use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct QuotePaymentRequest {
    pub job_name: String,
    pub quote_price: f64,
    pub required_deposit: f64,
    #[serde(default = "default_currency")]
    pub currency: String,
    #[serde(default)]
    pub memo: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct QuotePaymentLink {
    pub provider: &'static str,
    pub job_name: String,
    pub quote_price: f64,
    pub required_deposit: f64,
    pub currency: String,
    pub payment_link_id: String,
    pub payment_url: String,
}

fn default_currency() -> String {
    String::from("usd")
}
