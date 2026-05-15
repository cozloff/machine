#include <data/compression/arrow_nvcomp.h>

#include <arrow/api.h>
#include <arrow/io/file.h>
#include <cuda_runtime.h>
#include <nvcomp.hpp>
#include <nvcomp/lz4.hpp>
#include <parquet/arrow/reader.h>
#include <parquet/arrow/writer.h>
#include <parquet/properties.h>

#include <cstdio>
#include <filesystem>
#include <fstream>
#include <memory>
#include <stdexcept>
#include <string>
#include <vector>

namespace {

constexpr const char* kParquetPath = "data/mock_arrow.parquet";
constexpr const char* kCompressedPath = "data/mock_arrow.parquet.nvcomp_lz4";
constexpr const char* kRestoredPath = "data/mock_arrow.restored.parquet";
constexpr std::size_t kLz4ChunkSize = 1 << 20;

bool checkCuda(cudaError_t status, const char* operation) {
    if (status == cudaSuccess) {
        return true;
    }

    std::fprintf(stderr, "CUDA %s failed: %s\n", operation, cudaGetErrorString(status));
    return false;
}

bool checkArrow(const arrow::Status& status, const char* operation) {
    if (status.ok()) {
        return true;
    }

    std::fprintf(stderr, "Arrow %s failed: %s\n", operation, status.ToString().c_str());
    return false;
}

template <typename T>
bool assignArrowResult(arrow::Result<T> result, T* out, const char* operation) {
    if (!result.ok()) {
        std::fprintf(stderr, "Arrow %s failed: %s\n", operation, result.status().ToString().c_str());
        return false;
    }

    *out = std::move(result).ValueOrDie();
    return true;
}

std::vector<uint8_t> readAllBytes(const std::string& path) {
    std::ifstream input(path, std::ios::binary);
    if (!input) {
        std::fprintf(stderr, "Failed to open %s for reading.\n", path.c_str());
        return {};
    }

    return {std::istreambuf_iterator<char>(input), std::istreambuf_iterator<char>()};
}

bool writeAllBytes(const std::string& path, const std::vector<uint8_t>& bytes) {
    std::ofstream output(path, std::ios::binary);
    if (!output) {
        std::fprintf(stderr, "Failed to open %s for writing.\n", path.c_str());
        return false;
    }

    output.write(reinterpret_cast<const char*>(bytes.data()), static_cast<std::streamsize>(bytes.size()));
    return output.good();
}

bool buildSampleTable(std::shared_ptr<arrow::Table>* table) {
    arrow::Int32Builder id_builder;
    arrow::StringBuilder molecule_builder;
    arrow::DoubleBuilder score_builder;

    if (!checkArrow(id_builder.AppendValues({1, 2, 3, 4}), "append id values") ||
        !checkArrow(molecule_builder.AppendValues({"valproic acid", "CHIR99021", "forskolin", "DZNep"}),
                    "append molecule values") ||
        !checkArrow(score_builder.AppendValues({0.91, 0.84, 0.77, 0.72}), "append score values")) {
        return false;
    }

    std::shared_ptr<arrow::Array> ids;
    std::shared_ptr<arrow::Array> molecules;
    std::shared_ptr<arrow::Array> scores;
    if (!checkArrow(id_builder.Finish(&ids), "finish id array") ||
        !checkArrow(molecule_builder.Finish(&molecules), "finish molecule array") ||
        !checkArrow(score_builder.Finish(&scores), "finish score array")) {
        return false;
    }

    const std::shared_ptr<arrow::Schema> schema = arrow::schema({
        arrow::field("id", arrow::int32()),
        arrow::field("molecule", arrow::utf8()),
        arrow::field("score", arrow::float64()),
    });

    *table = arrow::Table::Make(schema, {ids, molecules, scores});
    return true;
}

bool writeParquetWithArrow(const std::shared_ptr<arrow::Table>& table) {
    std::shared_ptr<arrow::io::FileOutputStream> output;
    if (!assignArrowResult(arrow::io::FileOutputStream::Open(kParquetPath), &output, "open parquet output")) {
        return false;
    }

    const std::shared_ptr<parquet::WriterProperties> properties =
        parquet::WriterProperties::Builder()
            .compression(parquet::Compression::UNCOMPRESSED)
            ->build();

    return checkArrow(
        parquet::arrow::WriteTable(*table, arrow::default_memory_pool(), output, table->num_rows(), properties),
        "write parquet table");
}

bool readParquetAndPrint(const std::string& path) {
    std::shared_ptr<arrow::io::ReadableFile> input;
    if (!assignArrowResult(arrow::io::ReadableFile::Open(path), &input, "open parquet for read")) {
        return false;
    }

    std::unique_ptr<parquet::arrow::FileReader> reader;
    if (!assignArrowResult(parquet::arrow::OpenFile(input, arrow::default_memory_pool()),
                           &reader,
                           "open parquet reader")) {
        return false;
    }

    std::shared_ptr<arrow::Table> table;
    if (!assignArrowResult(reader->ReadTable(), &table, "read parquet table")) {
        return false;
    }

    const auto ids = std::static_pointer_cast<arrow::Int32Array>(table->column(0)->chunk(0));
    const auto molecules = std::static_pointer_cast<arrow::StringArray>(table->column(1)->chunk(0));
    const auto scores = std::static_pointer_cast<arrow::DoubleArray>(table->column(2)->chunk(0));

    std::printf("Read %lld rows from %s\n", static_cast<long long>(table->num_rows()), path.c_str());
    std::printf("  id | molecule      | score\n");
    for (int64_t row = 0; row < table->num_rows(); ++row) {
        std::printf("  %2d | %-13s | %.2f\n",
                    ids->Value(row),
                    molecules->GetString(row).c_str(),
                    scores->Value(row));
    }
    return true;
}

bool nvcompRoundTrip(const std::vector<uint8_t>& parquet_bytes, std::vector<uint8_t>* restored_bytes) {
    cudaStream_t stream = nullptr;
    uint8_t* device_input = nullptr;
    uint8_t* device_compressed = nullptr;
    uint8_t* device_restored = nullptr;
    cudaEvent_t compress_start = nullptr;
    cudaEvent_t compress_stop = nullptr;
    cudaEvent_t decompress_start = nullptr;
    cudaEvent_t decompress_stop = nullptr;

    bool ok = checkCuda(cudaStreamCreate(&stream), "create stream") &&
              checkCuda(cudaEventCreate(&compress_start), "create compress start") &&
              checkCuda(cudaEventCreate(&compress_stop), "create compress stop") &&
              checkCuda(cudaEventCreate(&decompress_start), "create decompress start") &&
              checkCuda(cudaEventCreate(&decompress_stop), "create decompress stop");
    if (!ok) return false;

    try {
        nvcomp::LZ4Manager manager{kLz4ChunkSize,
                                   nvcompBatchedLZ4CompressDefaultOpts,
                                   nvcompBatchedLZ4DecompressDefaultOpts,
                                   stream};
        const nvcomp::CompressionConfig compression_config = manager.configure_compression(parquet_bytes.size());

        ok = checkCuda(cudaMalloc(reinterpret_cast<void**>(&device_input), parquet_bytes.size()), "allocate input") &&
             checkCuda(cudaMalloc(reinterpret_cast<void**>(&device_compressed),
                                  compression_config.max_compressed_buffer_size),
                       "allocate compressed") &&
             checkCuda(cudaMemcpyAsync(device_input,
                                       parquet_bytes.data(),
                                       parquet_bytes.size(),
                                       cudaMemcpyHostToDevice,
                                       stream),
                       "copy parquet to GPU");
        if (!ok) throw std::runtime_error("CUDA setup failed");

        checkCuda(cudaEventRecord(compress_start, stream), "record compress start");
        manager.compress(device_input, device_compressed, compression_config);
        checkCuda(cudaEventRecord(compress_stop, stream), "record compress stop");
        checkCuda(cudaEventSynchronize(compress_stop), "sync compress stop");

        const std::size_t compressed_size = manager.get_compressed_output_size(device_compressed);
        std::vector<uint8_t> compressed_bytes(compressed_size);
        ok = checkCuda(cudaMemcpyAsync(compressed_bytes.data(),
                                       device_compressed,
                                       compressed_size,
                                       cudaMemcpyDeviceToHost,
                                       stream),
                       "copy compressed to host") &&
             checkCuda(cudaStreamSynchronize(stream), "sync compressed copy");
        if (!ok || !writeAllBytes(kCompressedPath, compressed_bytes)) {
            throw std::runtime_error("failed to persist compressed output");
        }

        const nvcomp::DecompressionConfig decompression_config =
            manager.configure_decompression(compression_config);
        ok = checkCuda(cudaMalloc(reinterpret_cast<void**>(&device_restored),
                                  decompression_config.decomp_data_size),
                       "allocate restored");
        if (!ok) throw std::runtime_error("CUDA decompression setup failed");

        checkCuda(cudaEventRecord(decompress_start, stream), "record decompress start");
        manager.decompress(device_restored, device_compressed, decompression_config);
        checkCuda(cudaEventRecord(decompress_stop, stream), "record decompress stop");
        checkCuda(cudaEventSynchronize(decompress_stop), "sync decompress stop");

        restored_bytes->resize(decompression_config.decomp_data_size);
        ok = checkCuda(cudaMemcpyAsync(restored_bytes->data(),
                                       device_restored,
                                       restored_bytes->size(),
                                       cudaMemcpyDeviceToHost,
                                       stream),
                       "copy restored to host") &&
             checkCuda(cudaStreamSynchronize(stream), "sync restored copy");
        if (!ok) throw std::runtime_error("failed to copy restored data");

        float compress_ms = 0.0f;
        float decompress_ms = 0.0f;
        checkCuda(cudaEventElapsedTime(&compress_ms, compress_start, compress_stop), "measure compression");
        checkCuda(cudaEventElapsedTime(&decompress_ms, decompress_start, decompress_stop), "measure decompression");

        const double mib = static_cast<double>(parquet_bytes.size()) / (1024.0 * 1024.0);
        std::printf("nvCOMP LZ4 compressed %zu bytes -> %zu bytes\n", parquet_bytes.size(), compressed_size);
        std::printf("  Ratio: %.2fx\n", static_cast<double>(parquet_bytes.size()) / compressed_size);
        std::printf("  Compression time: %.3f ms (%.2f MiB/s)\n", compress_ms, mib / (compress_ms / 1000.0));
        std::printf("  Decompression time: %.3f ms (%.2f MiB/s)\n", decompress_ms, mib / (decompress_ms / 1000.0));
    } catch (const std::exception& error) {
        std::fprintf(stderr, "nvCOMP Parquet round trip failed: %s\n", error.what());
        ok = false;
    }

    cudaEventDestroy(compress_start);
    cudaEventDestroy(compress_stop);
    cudaEventDestroy(decompress_start);
    cudaEventDestroy(decompress_stop);
    cudaFree(device_input);
    cudaFree(device_compressed);
    cudaFree(device_restored);
    cudaStreamDestroy(stream);
    return ok;
}

}  // namespace

bool createMockArrowParquet() {
    std::filesystem::create_directories("data");

    std::shared_ptr<arrow::Table> table;
    if (!buildSampleTable(&table) || !writeParquetWithArrow(table)) {
        return false;
    }

    const std::vector<uint8_t> bytes = readAllBytes(kParquetPath);
    if (bytes.empty()) return false;

    std::printf("Created mock Arrow Parquet file: %s\n", kParquetPath);
    std::printf("  Rows: %lld\n", static_cast<long long>(table->num_rows()));
    std::printf("  Columns: id:int32, molecule:utf8, score:double\n");
    std::printf("  Parquet internal compression: uncompressed\n");
    std::printf("  Size: %zu bytes\n", bytes.size());
    return true;
}

bool compressAndDecompressParquetWithNvcomp() {
    if (!std::filesystem::exists(kParquetPath) && !createMockArrowParquet()) {
        return false;
    }

    const std::vector<uint8_t> parquet_bytes = readAllBytes(kParquetPath);
    if (parquet_bytes.empty()) return false;

    std::vector<uint8_t> restored_bytes;
    if (!nvcompRoundTrip(parquet_bytes, &restored_bytes)) {
        return false;
    }

    if (restored_bytes != parquet_bytes) {
        std::fprintf(stderr, "Restored Parquet bytes do not match the original.\n");
        return false;
    }

    if (!writeAllBytes(kRestoredPath, restored_bytes)) {
        return false;
    }

    std::printf("Wrote compressed file: %s\n", kCompressedPath);
    std::printf("Wrote restored Parquet file: %s\n", kRestoredPath);
    return readParquetAndPrint(kRestoredPath);
}
