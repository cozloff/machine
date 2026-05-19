use crate::models::machine::{QuotePaymentLink, QuotePaymentRequest};

mod error;
mod stripe;

pub use error::MachineServiceError;
use stripe::StripeClient;

pub struct MachineService {
    stripe: StripeClient,
}

impl MachineService {
    pub fn new() -> Self {
        Self {
            stripe: StripeClient::from_env(),
        }
    }

    pub async fn create_quote_payment_link(
        &self,
        request: QuotePaymentRequest,
    ) -> Result<QuotePaymentLink, MachineServiceError> {
        validate_quote_payment_request(&request)?;
        self.stripe.create_payment_link(request).await
    }
}

impl Default for MachineService {
    fn default() -> Self {
        Self::new()
    }
}

fn validate_quote_payment_request(
    request: &QuotePaymentRequest,
) -> Result<(), MachineServiceError> {
    if request.job_name.trim().is_empty() {
        return Err(MachineServiceError::InvalidInput(
            "job_name must not be empty",
        ));
    }

    if request.quote_price <= 0.0 {
        return Err(MachineServiceError::InvalidInput(
            "quote_price must be positive",
        ));
    }

    if request.required_deposit <= 0.0 {
        return Err(MachineServiceError::InvalidInput(
            "required_deposit must be positive",
        ));
    }

    if request.currency.trim().is_empty() {
        return Err(MachineServiceError::InvalidInput(
            "currency must not be empty",
        ));
    }

    Ok(())
}
