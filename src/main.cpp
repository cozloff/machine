#include <data/compression/arrow_nvcomp.h>
#include <gpu/gpu_test.h>
#include <gpu/rho_guesser.h>
#include <ingest/pubchem.h>

#include <CLI/CLI.hpp>

#include <cstdio>

int main(int argc, char** argv) {
    CLI::App app{"machine"};
    app.require_subcommand(1, 1);

    CLI::App* ingest_cmd = app.add_subcommand("ingest", "Ingest candidate compounds from PubChem into sqlite.");
    CLI::App* data_cmd = app.add_subcommand("data", "Run data file and compression helpers.");
    data_cmd->require_subcommand(1, 1);

    CLI::App* parquet_cmd = data_cmd->add_subcommand("parquet", "Create and GPU-compress sample Parquet files.");
    parquet_cmd->require_subcommand(1, 1);
    CLI::App* parquet_create_cmd =
        parquet_cmd->add_subcommand("create", "Create data/mock_arrow.parquet with Arrow's Parquet writer.");
    CLI::App* parquet_nvcomp_cmd =
        parquet_cmd->add_subcommand("nvcomp", "Compress and decompress data/mock_arrow.parquet with nvCOMP LZ4.");
    CLI::App* gpu_cmd = app.add_subcommand("gpu", "Run CUDA helper commands.");
    gpu_cmd->require_subcommand(1, 1);

    CLI::App* gpu_report_cmd = gpu_cmd->add_subcommand("report", "Print CUDA device capabilities.");
    CLI::App* rho_guess_cmd = gpu_cmd->add_subcommand("rho-guess", "Run the CUDA reciprocal-space rho guess demo.");

    if (argc == 1) {
        std::printf("%s", app.help().c_str());
        return 0;
    }

    CLI11_PARSE(app, argc, argv);

    if (ingest_cmd->parsed()) {
        return ingestPubChem() ? 0 : 1;
    }

    if (parquet_create_cmd->parsed()) {
        return createMockArrowParquet() ? 0 : 1;
    }

    if (parquet_nvcomp_cmd->parsed()) {
        return compressAndDecompressParquetWithNvcomp() ? 0 : 1;
    }

    if (gpu_report_cmd->parsed()) {
        return runGpuTest() ? 0 : 1;
    }

    if (rho_guess_cmd->parsed()) {
        return runRhoGuessDemo() ? 0 : 1;
    }

    std::fprintf(stderr, "No command selected.\n");
    return 1;
}
