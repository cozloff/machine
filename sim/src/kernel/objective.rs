use crate::kernel::policy::QuoteDecision;
use crate::kernel::reality::RealitySignal;

#[derive(Debug)]
pub struct GoalFunction {
    pub name: &'static str,
    pub purpose: &'static str,
    pub weights: RewardWeights,
}

#[derive(Debug)]
pub struct RewardWeights {
    pub persistence: f64,
    pub wealth: f64,
    pub control: f64,
    pub risk: f64,
    pub self_deception: f64,
}

#[derive(Debug)]
pub struct RewardScore {
    pub persistence_delta: f64,
    pub wealth_delta: f64,
    pub control_delta: f64,
    pub risk: f64,
    pub self_deception: f64,
    pub total_reward: f64,
}

impl GoalFunction {
    pub fn agent_1_reality_optimization() -> Self {
        Self {
            name: "agent_1_reality_optimization",
            purpose: "maximize persistence_time, control_surface, and reality_corrected wealth",
            weights: RewardWeights {
                persistence: 1.0,
                wealth: 1.0,
                control: 1.0,
                risk: 1.0,
                self_deception: 1.0,
            },
        }
    }

    pub fn score(&self, decision: &QuoteDecision, signal: &RealitySignal) -> RewardScore {
        let wealth_delta = signal.observation.as_ref().map_or_else(
            || decision.average_profit(),
            |observation| observation.observed_profit,
        );
        let cash_shortfall = signal.observation.as_ref().map_or_else(
            || decision.average_cash_shortfall(),
            |observation| observation.observed_cash_shortfall,
        );
        let persistence_delta = wealth_delta - cash_shortfall;
        let control_delta = wealth_delta / (1.0 + cash_shortfall);
        let risk =
            decision.loss_probability() + decision.cash_shortfall_probability() + cash_shortfall;
        let self_deception = if signal.is_open_loop() { 1.0 } else { 0.0 };

        let total_reward = (self.weights.persistence * persistence_delta)
            + (self.weights.wealth * wealth_delta)
            + (self.weights.control * control_delta)
            - (self.weights.risk * risk)
            - (self.weights.self_deception * self_deception);

        RewardScore {
            persistence_delta,
            wealth_delta,
            control_delta,
            risk,
            self_deception,
            total_reward,
        }
    }
}
