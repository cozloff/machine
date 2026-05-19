pub struct StripeClient {
    client: Client,
    secret_key: Option<String>,
}

impl StripeClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            secret_key: std::env::var("STRIPE_SECRET_KEY").ok(),
        }
    }
}