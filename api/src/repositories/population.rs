use std::{
    error::Error,
    fmt, fs,
    time::{SystemTime, UNIX_EPOCH},
};

use sea_orm::{
    ActiveModelBehavior, ActiveValue::Set, ConnectionTrait, DatabaseConnection, DbErr,
    DeriveEntityModel, DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Schema,
    TransactionTrait, entity::prelude::*, sea_query::OnConflict,
};

use crate::{models::population::PopulationSnapshot, repositories::sqlite::SqliteDatabase};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "population_snapshots")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub country_code: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub year: String,
    pub country_name: Option<String>,
    pub total: Option<f64>,
    pub growth_annual_percent: Option<f64>,
    pub density_per_sq_km: Option<f64>,
    pub urban_total: Option<f64>,
    pub urban_percent: Option<f64>,
    pub rural_total: Option<f64>,
    pub rural_percent: Option<f64>,
    pub female_total: Option<f64>,
    pub male_total: Option<f64>,
    pub age_0_to_14_total: Option<f64>,
    pub age_15_to_64_total: Option<f64>,
    pub age_65_plus_total: Option<f64>,
    pub birth_rate_per_1000: Option<f64>,
    pub death_rate_per_1000: Option<f64>,
    pub fertility_rate: Option<f64>,
    pub life_expectancy_years: Option<f64>,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

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
        if let Some(parent) = self.database.path().parent() {
            fs::create_dir_all(parent)?;
        }

        let connection = self.database.connect().await?;
        create_schema(&connection).await?;

        let transaction = connection.begin().await?;
        for snapshot in snapshots {
            save_snapshot(&transaction, &snapshot).await?;
        }
        transaction.commit().await?;

        Ok(())
    }
}

async fn create_schema(connection: &DatabaseConnection) -> Result<(), PopulationRepositoryError> {
    let backend = connection.get_database_backend();
    let schema = Schema::new(backend);
    let statement = backend.build(schema.create_table_from_entity(Entity).if_not_exists());

    connection.execute(statement).await?;

    Ok(())
}

async fn save_snapshot<C>(
    connection: &C,
    snapshot: &PopulationSnapshot,
) -> Result<(), PopulationRepositoryError>
where
    C: ConnectionTrait,
{
    let year = snapshot
        .year
        .as_deref()
        .ok_or_else(|| PopulationRepositoryError::MissingYear {
            country_code: snapshot.country_code.clone(),
        })?;

    let active_model = ActiveModel {
        country_code: Set(snapshot.country_code.clone()),
        year: Set(year.to_string()),
        country_name: Set(snapshot.country_name.clone()),
        total: Set(snapshot.total),
        growth_annual_percent: Set(snapshot.growth_annual_percent),
        density_per_sq_km: Set(snapshot.density_per_sq_km),
        urban_total: Set(snapshot.urban_total),
        urban_percent: Set(snapshot.urban_percent),
        rural_total: Set(snapshot.rural_total),
        rural_percent: Set(snapshot.rural_percent),
        female_total: Set(snapshot.female_total),
        male_total: Set(snapshot.male_total),
        age_0_to_14_total: Set(snapshot.age_0_to_14_total),
        age_15_to_64_total: Set(snapshot.age_15_to_64_total),
        age_65_plus_total: Set(snapshot.age_65_plus_total),
        birth_rate_per_1000: Set(snapshot.birth_rate_per_1000),
        death_rate_per_1000: Set(snapshot.death_rate_per_1000),
        fertility_rate: Set(snapshot.fertility_rate),
        life_expectancy_years: Set(snapshot.life_expectancy_years),
        updated_at: Set(current_timestamp()),
    };

    Entity::insert(active_model)
        .on_conflict(
            OnConflict::columns([Column::CountryCode, Column::Year])
                .update_columns([
                    Column::CountryName,
                    Column::Total,
                    Column::GrowthAnnualPercent,
                    Column::DensityPerSqKm,
                    Column::UrbanTotal,
                    Column::UrbanPercent,
                    Column::RuralTotal,
                    Column::RuralPercent,
                    Column::FemaleTotal,
                    Column::MaleTotal,
                    Column::Age0To14Total,
                    Column::Age15To64Total,
                    Column::Age65PlusTotal,
                    Column::BirthRatePer1000,
                    Column::DeathRatePer1000,
                    Column::FertilityRate,
                    Column::LifeExpectancyYears,
                    Column::UpdatedAt,
                ])
                .to_owned(),
        )
        .exec(connection)
        .await?;

    Ok(())
}

fn current_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

#[derive(Debug)]
pub enum PopulationRepositoryError {
    Database(DbErr),
    Io(std::io::Error),
    MissingYear { country_code: String },
}

impl From<DbErr> for PopulationRepositoryError {
    fn from(error: DbErr) -> Self {
        Self::Database(error)
    }
}

impl From<std::io::Error> for PopulationRepositoryError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl fmt::Display for PopulationRepositoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(error) => write!(formatter, "database error: {error}"),
            Self::Io(error) => write!(formatter, "filesystem error: {error}"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn saves_population_snapshot_to_sqlite() {
        let path = std::env::temp_dir().join(format!(
            "machine-population-{}-{}.sqlite",
            std::process::id(),
            current_timestamp()
        ));
        let _ = fs::remove_file(&path);

        let repository = PopulationRepository::new(SqliteDatabase::new(&path));
        let mut snapshot = PopulationSnapshot::new("USA".to_string());
        snapshot.country_name = Some("United States".to_string());
        snapshot.year = Some("2024".to_string());
        snapshot.total = Some(340_000_000.0);

        repository.save_snapshots(vec![snapshot]).await.unwrap();

        let connection = SqliteDatabase::new(&path).connect().await.unwrap();
        let row = Entity::find_by_id(("USA".to_string(), "2024".to_string()))
            .one(&connection)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(row.country_name.as_deref(), Some("United States"));
        assert_eq!(row.total, Some(340_000_000.0));

        let _ = fs::remove_file(path);
    }
}
