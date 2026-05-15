Commands: 

```bash
make up        # build/start dev container
make rebuild   # configure + compile + run
make build     # compile only
make run       # run existing binary
make shell     # open shell in container
make down      # stop it
```

Get into Sqlite:

```bash
sqlite3 data/machine.db
```

To install Windows Dependencies:

```bash
choco install cmake
choco install make
make deps-windows
```

CLI App: 

```bash
make run ARGS="ingest"
make run ARGS="gpu rho-guess"
make run ARGS="gpu report"
make run ARGS="data parquet create"
make run ARGS="data parquet nvcomp"
```

Change test 2