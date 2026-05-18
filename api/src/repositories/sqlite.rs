use std::path::{Path, PathBuf};

use sea_orm::{Database, DatabaseConnection, DbErr};

#[derive(Clone, Debug)]
pub struct SqliteDatabase {
    path: PathBuf,
}

impl SqliteDatabase {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn population_data() -> Self {
        Self::new(default_population_database_path())
    }

    pub async fn connect(&self) -> Result<DatabaseConnection, DbErr> {
        Database::connect(format!("sqlite://{}?mode=rwc", self.path.to_string_lossy())).await
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

fn default_population_database_path() -> PathBuf {
    std::env::var_os("POPULATION_DATABASE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .map(|root| root.join("data/population.sqlite"))
                .unwrap_or_else(|| PathBuf::from("data/population.sqlite"))
        })
}
