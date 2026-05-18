use std::{error::Error, fmt, fs};

use rusqlite::{Connection, params};

use crate::{models::population::PopulationSnapshot, repositories::sqlite::SqliteDatabase};

#[derive(Clone, Debug)]
pub struct PopulationRepository {
    database: SqliteDatabase,
}

impl PopulationRepository {
    pub fn new(database: SqliteDatabase) -> Self {
        Self { database }
    }

    pub fn sqlite() -> Self {
        Self::new(SqliteDatabase::population_data())
    }

    pub async fn save_snapshots(
        &self,
        snapshots: Vec<PopulationSnapshot>,
    ) -> Result<(), PopulationRepositoryError> {
        let database = self.database.clone();

        tokio::task::spawn_blocking(move || {
            if let Some(parent) = database.path().parent() {
                fs::create_dir_all(parent)?;
            }

            let mut connection = database.connect()?;
            create_schema(&connection)?;

            let transaction = connection.transaction()?;
            for snapshot in snapshots {
                save_snapshot(&transaction, &snapshot)?;
            }
            transaction.commit()?;

            Ok(())
        })
        .await?
    }
}

fn create_schema(connection: &Connection) -> Result<(), PopulationRepositoryError> {
    connection.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS population_snapshots (
            country_code TEXT NOT NULL,
            year TEXT NOT NULL,
            country_name TEXT,
            total REAL,
            growth_annual_percent REAL,
            density_per_sq_km REAL,
            urban_total REAL,
            urban_percent REAL,
            rural_total REAL,
            rural_percent REAL,
            female_total REAL,
            male_total REAL,
            age_0_to_14_total REAL,
            age_15_to_64_total REAL,
            age_65_plus_total REAL,
            birth_rate_per_1000 REAL,
            death_rate_per_1000 REAL,
            fertility_rate REAL,
            life_expectancy_years REAL,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (country_code, year)
        );
        ",
    )?;

    Ok(())
}

fn save_snapshot(
    connection: &Connection,
    snapshot: &PopulationSnapshot,
) -> Result<(), PopulationRepositoryError> {
    let year = snapshot
        .year
        .as_deref()
        .ok_or_else(|| PopulationRepositoryError::MissingYear {
            country_code: snapshot.country_code.clone(),
        })?;

    connection.execute(
        "
        INSERT INTO population_snapshots (
            country_code,
            year,
            country_name,
            total,
            growth_annual_percent,
            density_per_sq_km,
            urban_total,
            urban_percent,
            rural_total,
            rural_percent,
            female_total,
            male_total,
            age_0_to_14_total,
            age_15_to_64_total,
            age_65_plus_total,
            birth_rate_per_1000,
            death_rate_per_1000,
            fertility_rate,
            life_expectancy_years,
            updated_at
        )
        VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
            ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19,
            CURRENT_TIMESTAMP
        )
        ON CONFLICT(country_code, year) DO UPDATE SET
            country_name = excluded.country_name,
            total = excluded.total,
            growth_annual_percent = excluded.growth_annual_percent,
            density_per_sq_km = excluded.density_per_sq_km,
            urban_total = excluded.urban_total,
            urban_percent = excluded.urban_percent,
            rural_total = excluded.rural_total,
            rural_percent = excluded.rural_percent,
            female_total = excluded.female_total,
            male_total = excluded.male_total,
            age_0_to_14_total = excluded.age_0_to_14_total,
            age_15_to_64_total = excluded.age_15_to_64_total,
            age_65_plus_total = excluded.age_65_plus_total,
            birth_rate_per_1000 = excluded.birth_rate_per_1000,
            death_rate_per_1000 = excluded.death_rate_per_1000,
            fertility_rate = excluded.fertility_rate,
            life_expectancy_years = excluded.life_expectancy_years,
            updated_at = CURRENT_TIMESTAMP
        ",
        params![
            snapshot.country_code,
            year,
            snapshot.country_name,
            snapshot.total,
            snapshot.growth_annual_percent,
            snapshot.density_per_sq_km,
            snapshot.urban_total,
            snapshot.urban_percent,
            snapshot.rural_total,
            snapshot.rural_percent,
            snapshot.female_total,
            snapshot.male_total,
            snapshot.age_0_to_14_total,
            snapshot.age_15_to_64_total,
            snapshot.age_65_plus_total,
            snapshot.birth_rate_per_1000,
            snapshot.death_rate_per_1000,
            snapshot.fertility_rate,
            snapshot.life_expectancy_years,
        ],
    )?;

    Ok(())
}

#[derive(Debug)]
pub enum PopulationRepositoryError {
    Database(rusqlite::Error),
    Io(std::io::Error),
    Join(tokio::task::JoinError),
    MissingYear { country_code: String },
}

impl From<rusqlite::Error> for PopulationRepositoryError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Database(error)
    }
}

impl From<std::io::Error> for PopulationRepositoryError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<tokio::task::JoinError> for PopulationRepositoryError {
    fn from(error: tokio::task::JoinError) -> Self {
        Self::Join(error)
    }
}

impl fmt::Display for PopulationRepositoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(error) => write!(formatter, "database error: {error}"),
            Self::Io(error) => write!(formatter, "filesystem error: {error}"),
            Self::Join(error) => write!(formatter, "database task failed: {error}"),
            Self::MissingYear { country_code } => {
                write!(
                    formatter,
                    "population snapshot for {country_code} is missing a year"
                )
            }
        }
    }
}

impl Error for PopulationRepositoryError {}
