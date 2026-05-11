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
choco install sqlite
```