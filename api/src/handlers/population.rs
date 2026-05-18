use axum::{Json, extract::Path};

use crate::handlers::error::{ApiError, ApiServiceError};
use crate::models::population::{PopulationRecord, PopulationSnapshot};
use crate::services::population::{PopulationService, PopulationServiceError};

pub async fn all_population() -> Result<Json<Vec<PopulationSnapshot>>, ApiError> {
    let service = PopulationService::new();

    service
        .snapshots_for_all_countries()
        .await
        .map(Json)
        .map_err(ApiError::from)
}

pub async fn save_all_population() -> Result<(), ApiError> {
    let service = PopulationService::new();

    service
        .save_all_countries_snapshot()
        .await
        .map_err(ApiError::from)
}

pub async fn latest_population(
    Path(country_code): Path<String>,
) -> Result<Json<PopulationSnapshot>, ApiError> {
    let service = PopulationService::new();

    service
        .snapshot_for_country(&country_code)
        .await?
        .map(Json)
        .ok_or_else(|| ApiError::not_found("population data not found"))
}

pub async fn latest_population_total(
    Path(country_code): Path<String>,
) -> Result<Json<PopulationRecord>, ApiError> {
    let service = PopulationService::new();

    service
        .latest_for_country(&country_code)
        .await?
        .map(Json)
        .ok_or_else(|| ApiError::not_found("population data not found"))
}

impl ApiServiceError for PopulationServiceError {
    const SERVICE: &'static str = "population";
}
