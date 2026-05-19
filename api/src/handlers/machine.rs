use axum::Json;

use crate::handlers::error::ApiError;
use crate::models::machine::{QuotePaymentLink, QuotePaymentRequest};
use crate::services::machine::{MachineService, MachineServiceError};

pub async fn create_quote_payment_link(
    Json(request): Json<QuotePaymentRequest>,
) -> Result<Json<QuotePaymentLink>, ApiError> {
    let service = MachineService::new();

    service
        .create_quote_payment_link(request)
        .await
        .map(Json)
        .map_err(machine_error)
}

fn machine_error(error: MachineServiceError) -> ApiError {
    match error {
        MachineServiceError::InvalidInput(message) => ApiError::bad_request(message),
        MachineServiceError::Config(message) => ApiError::service_unavailable(message),
        error => ApiError::bad_gateway("machine", error),
    }
}
