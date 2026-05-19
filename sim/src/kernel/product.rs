#[derive(Debug)]
pub struct ProductLoop {
    pub first_product: &'static str,
    pub domain: &'static str,
    pub input: &'static str,
    pub simulation: &'static str,
    pub ai_layer: &'static str,
    pub reality_signal: &'static str,
    pub output: &'static str,
}

impl ProductLoop {
    pub fn quote_risk_engine() -> Self {
        Self {
            first_product: "quote_risk_engine",
            domain: "precision_manufacturing",
            input: "job|machine|material|supplier|cash|customer",
            simulation: "monte_carlo_quote_risk",
            ai_layer: "semantic_parser|risk_explainer|policy_generator",
            reality_signal: "accepted|rejected|countered|paid|scrap|late|profit",
            output: "risk_adjusted_quote|model_update|wealth_delta",
        }
    }

    pub fn ascii_loop(&self) -> &'static str {
        "agent_1 -> ai -> product -> world_signal -> model_update -> wealth_delta"
    }
}
