# Machine

## Getting started

```bash
docker compose up
```
Then open another terminal:

```bash
docker exec -it jumpbox bash
```

Check that the CLI is available:

```bash
gum test
```

## Commands

```bash
gum mach up                 # build/start the dev container
gum mach rebuild            # reconfigure, compile, and run inside the container
gum mach ingest             # run PubChem ingestion
gum mach gpu                # print CUDA device capabilities
gum mach rho                # run the CUDA rho guess demo
gum mach parquet create     # create data/mock_arrow.parquet
gum mach parquet nvcomp     # compress/decompress Parquet with nvCOMP
gum mach down               # stop the dev container
```

The container must be running before commands that execute the compiled
`machine` binary:

```bash
gum mach up
gum mach rebuild
gum mach ingest
```

## SQLite

```bash
sqlite3 data/machine.db
```

## Windows Dependencies

```bash
choco install rust
```

The Windows dependency helper still lives in the Makefile. The day-to-day
project workflow uses `gum`.
