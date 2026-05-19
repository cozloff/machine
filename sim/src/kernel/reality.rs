use crate::kernel::boundary::{QuoteRequest, RealityObservationInput};
use crate::kernel::policy::QuoteDecision;

#[derive(Debug)]
pub struct Prediction {
    pub expected_cost: f64,
    pub expected_profit: f64,
    pub expected_cash_shortfall: f64,
}

#[derive(Debug)]
pub struct RealityObservation {
    pub customer_response: String,
    pub observed_cost: f64,
    pub observed_profit: f64,
    pub observed_cash_shortfall: f64,
    pub scrap: Option<bool>,
    pub rework: Option<bool>,
    pub late: Option<bool>,
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

impl RealityObservation {
    pub fn from_input(input: &RealityObservationInput, request: &QuoteRequest) -> Self {
        let observed_profit = input.actual_cash_received - input.actual_total_cost;
        let observed_cash_shortfall = input
            .actual_cash_shortfall
            .unwrap_or_else(|| f64::max(0.0, input.actual_total_cost - request.cash_on_hand));

        Self {
            customer_response: input.customer_response.clone(),
            observed_cost: input.actual_total_cost,
            observed_profit,
            observed_cash_shortfall,
            scrap: input.scrap,
            rework: input.rework,
            late: input.late,
        }
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
