## Repo Workflow

`gum` is the project CLI. It wraps the Docker Compose dev container and the
compiled `machine` binary, so normal workflows do not need `make` or the old
`bin/machine` wrapper.

## Prequisites

Git bash **Rust**: 
1. Download **[rustup-init.exe](https://www.google.com/url?sa=i&source=web&rct=j&url=https://rustup.rs/&ved=2ahUKEwiT9en2-cOUAxWqJkQIHcJBCWMQy_kOegoIAggACAEIERAC&opi=89978449&cd&psig=AOvVaw1n4D33NpzDGIH89I0VJZE4&ust=1779232022846000)**
2. Run: 
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Install

```bash
chmod +x path.sh
./path.sh
```

Check that the CLI is available:

```bash
gum --help
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

## Simulation Prototype

The `sim` folder contains a Rust Monte Carlo prototype. Today it models stock
collateral shortfall risk. The next useful step is to adapt the same simulation
pattern into manufacturing quote risk:

```text
stock price path
  -> material price / cycle time / scrap paths

collateral shortfall
  -> job loss probability / cash shortfall / quote recommendation
```

Run or develop it from:

```bash
cd sim
cargo run
```

## SQLite

```bash
sqlite3 data/machine.db
```

The Windows dependency helper still lives in the Makefile. The day-to-day
project workflow uses `gum`.