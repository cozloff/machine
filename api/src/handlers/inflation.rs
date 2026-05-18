use axum::{Json, extract::Path};

use crate::handlers::error::{ApiError, ApiServiceError};
use crate::models::inflation::{InflationRateRecord, InflationRateSnapshot};
use crate::services::inflation::{InflationService, InflationServiceError};

pub async fn all_inflation() -> Result<Json<Vec<InflationRateSnapshot>>, ApiError> {
    let service = InflationService::new();

    service
        .snapshots_for_all_countries()
        .await
        .map(Json)
        .map_err(ApiError::from)
}

pub async fn latest_inflation(
    Path(country_code): Path<String>,
) -> Result<Json<InflationRateSnapshot>, ApiError> {
    let service = InflationService::new();

    service
        .snapshot_for_country(&country_code)
        .await?
        .map(Json)
        .ok_or_else(|| ApiError::not_found("inflation data not found"))
}

pub async fn latest_inflation_rate(
    Path(country_code): Path<String>,
) -> Result<Json<InflationRateRecord>, ApiError> {
    let service = InflationService::new();

    service
        .latest_for_country(&country_code)
        .await?
        .map(Json)
        .ok_or_else(|| ApiError::not_found("inflation data not found"))
}

impl ApiServiceError for InflationServiceError {
    const SERVICE: &'static str = "inflation";
}
