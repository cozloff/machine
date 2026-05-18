Data
====

Runtime data lives in this directory.

## Population SQLite Database

The API stores saved population snapshots in:

```text
data/population.sqlite
```

The default path is resolved from the API crate root, so running the API from
`api/` still writes to the repository-level `data/` directory. Set
`POPULATION_DATABASE_PATH` to override the location.

The `population_snapshots` table is created automatically by the population
repository the first time snapshots are saved.
