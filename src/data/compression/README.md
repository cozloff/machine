This directory is for Arrow/Parquet and GPU compression experiments.

- `arrow_nvcomp.cpp` creates `data/mock_arrow.parquet` with Apache Arrow's
  Parquet writer.
- The Parquet file is intentionally written without internal Parquet
  compression so nvCOMP can compress the file bytes directly.
- nvCOMP LZ4 is used for high-throughput GPU compression/decompression,
  producing `data/mock_arrow.parquet.nvcomp_lz4` and verifying a restored
  `data/mock_arrow.restored.parquet`.
