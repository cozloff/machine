use crate::models::machine::{QuotePaymentLink, QuotePaymentRequest};
use crate::services::machine::MachineServiceError;
use reqwest::Client;
use serde::Deserialize;

const STRIPE_PAYMENT_LINKS_URL: &str = "https://api.stripe.com/v1/payment_links";

pub struct StripeClient {
    client: Client,
    secret_key: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StripePaymentLinkResponse {
    id: String,
    url: String,
}

impl StripeClient {
    pub fn from_env() -> Self {
        Self {
            client: Client::new(),
            secret_key: std::env::var("STRIPE_SECRET_KEY").ok(),
        }
    }

    pub async fn create_payment_link(
        &self,
        request: QuotePaymentRequest,
    ) -> Result<QuotePaymentLink, MachineServiceError> {
        let secret_key = self
            .secret_key
            .as_deref()
            .ok_or(MachineServiceError::Config("STRIPE_SECRET_KEY is required"))?;

        let amount_cents = to_minor_units(request.required_deposit);
        let memo = request
            .memo
            .as_deref()
            .unwrap_or("deposit required to begin work");
        let product_name = format!("{} deposit", request.job_name);

        let form = vec![
            (
                "line_items[0][price_data][currency]".to_string(),
                request.currency.to_lowercase(),
            ),
            (
                "line_items[0][price_data][unit_amount]".to_string(),
                amount_cents.to_string(),
            ),
            (
                "line_items[0][price_data][product_data][name]".to_string(),
                product_name,
            ),
            (
                "line_items[0][price_data][product_data][description]".to_string(),
                memo.to_string(),
            ),
            ("line_items[0][quantity]".to_string(), String::from("1")),
            ("metadata[job_name]".to_string(), request.job_name.clone()),
            (
                "metadata[quote_price]".to_string(),
                format!("{:.2}", request.quote_price),
            ),
            (
                "metadata[required_deposit]".to_string(),
                format!("{:.2}", request.required_deposit),
            ),
        ];

        let response = self
            .client
            .post(STRIPE_PAYMENT_LINKS_URL)
            .basic_auth(secret_key, Some(""))
            .form(&form)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(MachineServiceError::UnexpectedStatus { status, body });
        }

        let stripe_response: StripePaymentLinkResponse =
            serde_json::from_str(&body).map_err(|_| MachineServiceError::UnexpectedStatus {
                status,
                body: String::from("Stripe response did not include expected payment link fields"),
            })?;

        Ok(QuotePaymentLink {
            provider: "stripe_payment_links_api",
            job_name: request.job_name,
            quote_price: request.quote_price,
            required_deposit: request.required_deposit,
            currency: request.currency.to_lowercase(),
            payment_link_id: stripe_response.id,
            payment_url: stripe_response.url,
        })
    }
}

fn to_minor_units(amount: f64) -> i64 {
    (amount * 100.0).round() as i64
}
