use crate::kernel::policy::QuoteDecision;

#[derive(Debug)]
pub struct Prediction {
    pub expected_cost: f64,
    pub expected_profit: f64,
    pub expected_cash_shortfall: f64,
}

#[derive(Debug)]
pub struct RealityObservation {
    pub observed_cost: f64,
    pub observed_profit: f64,
    pub observed_cash_shortfall: f64,
}

#[derive(Debug)]
pub struct PredictionError {
    pub cost_error: f64,
    pub profit_error: f64,
    pub cash_shortfall_error: f64,
}

#[derive(Debug)]
pub struct ModelUpdate {
    pub status: FeedbackStatus,
    pub confidence_pressure: f64,
    pub error: Option<PredictionError>,
}

#[derive(Debug)]
pub enum FeedbackStatus {
    OpenLoop,
    RealityMeasured,
}

#[derive(Debug)]
pub struct RealitySignal {
    pub prediction: Prediction,
    pub observation: Option<RealityObservation>,
    pub update: ModelUpdate,
}

impl RealitySignal {
    pub fn open_loop(decision: &QuoteDecision) -> Self {
        let prediction = Prediction::from_decision(decision);

        Self {
            prediction,
            observation: None,
            update: ModelUpdate {
                status: FeedbackStatus::OpenLoop,
                confidence_pressure: 0.0,
                error: None,
            },
        }
    }

    pub fn measure(decision: &QuoteDecision, observation: RealityObservation) -> Self {
        let prediction = Prediction::from_decision(decision);
        let error = PredictionError {
            cost_error: observation.observed_cost - prediction.expected_cost,
            profit_error: observation.observed_profit - prediction.expected_profit,
            cash_shortfall_error: observation.observed_cash_shortfall
                - prediction.expected_cash_shortfall,
        };
        let confidence_pressure = 1.0 / (1.0 + error.cost_error.abs());

        Self {
            prediction,
            observation: Some(observation),
            update: ModelUpdate {
                status: FeedbackStatus::RealityMeasured,
                confidence_pressure,
                error: Some(error),
            },
        }
    }

    pub fn is_open_loop(&self) -> bool {
        matches!(self.update.status, FeedbackStatus::OpenLoop)
    }
}

impl Prediction {
    fn from_decision(decision: &QuoteDecision) -> Self {
        Self {
            expected_cost: decision.average_cost(),
            expected_profit: decision.average_profit(),
            expected_cash_shortfall: decision.average_cash_shortfall(),
        }
    }
}
