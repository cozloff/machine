use crate::kernel::agent::AgentBoundary;
use crate::kernel::boundary::QuoteRequest;
use crate::kernel::currency::CurrencyConsequence;
use crate::kernel::execution::StageTiming;
use crate::kernel::information::ShockState;
use crate::kernel::physical_form::ManufacturingConsequence;
use rand::thread_rng;
use std::time::Instant;

const RISK_QUANTILE_Z: f64 = 1.65;

#[derive(Debug, Default)]
pub struct RiskTrace {
    simulations_with_loss: usize,
    simulations_with_cash_shortfall: usize,
    total_profit: f64,
    total_cost: f64,
    total_cost_squared: f64,
    total_cash_shortfall: f64,
    worst_loss: f64,
    max_cash_shortfall: f64,
}

#[derive(Debug)]
pub struct QuoteDecision {
    average_cost: f64,
    cost_std_dev: f64,
    average_profit: f64,
    expected_margin: f64,
    break_even_quote: f64,
    risk_adjusted_quote: f64,
    loss_probability: f64,
    worst_loss: f64,
    cash_shortfall_probability: f64,
    average_cash_shortfall: f64,
    max_cash_shortfall: f64,
    survival_delta: f64,
    control_surface: f64,
}

pub struct RiskEngine {
    agent: AgentBoundary,
    request: QuoteRequest,
}

impl RiskEngine {
    pub fn new(request: QuoteRequest) -> Self {
        request.validate();
        Self {
            agent: AgentBoundary::first_contributor(),
            request,
        }
    }

    pub fn agent(&self) -> &AgentBoundary {
        &self.agent
    }

    pub fn request(&self) -> &QuoteRequest {
        &self.request
    }

    pub fn simulate(&self, timing: &mut StageTiming) -> RiskTrace {
        let simulation_started = Instant::now();
        let mut rng = thread_rng();
        let mut trace = RiskTrace::default();

        for _ in 0..self.request.simulations {
            let sampling_started = Instant::now();
            let shocks = ShockState::sample(&self.request, &mut rng);
            timing.random_sampling += sampling_started.elapsed();

            let physical_started = Instant::now();
            let manufacturing = ManufacturingConsequence::from_shocks(&self.request, &shocks);
            timing.physical_cost_model += physical_started.elapsed();

            let currency_started = Instant::now();
            let currency = CurrencyConsequence::from_manufacturing(&self.request, &manufacturing);
            timing.currency_risk += currency_started.elapsed();

            let aggregation_started = Instant::now();
            trace.observe(&currency);
            timing.aggregation += aggregation_started.elapsed();
        }

        timing.simulation = simulation_started.elapsed();
        trace
    }

    pub fn decide(&self, trace: &RiskTrace) -> QuoteDecision {
        let simulations = self.request.simulations as f64;
        let average_profit = trace.total_profit / simulations;
        let average_cost = trace.total_cost / simulations;
        let cost_variance = (trace.total_cost_squared / simulations) - average_cost.powi(2);
        let cost_std_dev = cost_variance.max(0.0).sqrt();
        let loss_probability = trace.simulations_with_loss as f64 / simulations;
        let cash_shortfall_probability = trace.simulations_with_cash_shortfall as f64 / simulations;
        let average_cash_shortfall = trace.total_cash_shortfall / simulations;

        QuoteDecision {
            average_cost,
            cost_std_dev,
            average_profit,
            expected_margin: average_profit / self.request.quote_price,
            break_even_quote: average_cost,
            risk_adjusted_quote: average_cost + (RISK_QUANTILE_Z * cost_std_dev),
            loss_probability,
            worst_loss: trace.worst_loss,
            cash_shortfall_probability,
            average_cash_shortfall,
            max_cash_shortfall: trace.max_cash_shortfall,
            survival_delta: average_profit - average_cash_shortfall,
            control_surface: control_surface(average_profit, average_cash_shortfall),
        }
    }
}

impl RiskTrace {
    fn observe(&mut self, currency: &CurrencyConsequence) {
        if currency.profit < 0.0 {
            self.simulations_with_loss += 1;
            self.worst_loss = self.worst_loss.max(-currency.profit);
        }

        if currency.cash_shortfall > 0.0 {
            self.simulations_with_cash_shortfall += 1;
            self.max_cash_shortfall = self.max_cash_shortfall.max(currency.cash_shortfall);
        }

        self.total_profit += currency.profit;
        self.total_cost += currency.total_cost;
        self.total_cost_squared += currency.total_cost.powi(2);
        self.total_cash_shortfall += currency.cash_shortfall;
    }
}

pub fn print_report(agent: &AgentBoundary, request: &QuoteRequest, decision: &QuoteDecision) {
    println!("Accountable boundary");
    println!("  identity: {}", agent.identity);
    println!("  name: {}", agent.name);
    println!("  origin transform: {}", agent.origin_transform);
    println!("  objective: {}", agent.kernel_line());
    println!(
        "  drive: persistence_time={}, control_surface={}, optionality={}",
        agent.objective.persistence_time,
        agent.objective.control_surface,
        agent.objective.optionality
    );
    println!();
    println!("Manufacturing quote risk summary");
    println!("  job: {}", request.job_name);
    println!("  material: {}", request.material_name);
    println!("  simulations: {}", request.simulations);
    println!("  days until paid: {}", request.days_until_paid);
    println!();
    println!("Quote economics");
    println!("  quoted price: ${:.2}", request.quote_price);
    println!("  expected cost: ${:.2}", decision.average_cost);
    println!("  cost std dev: ${:.2}", decision.cost_std_dev);
    println!("  expected profit: ${:.2}", decision.average_profit);
    println!(
        "  expected margin: {:.2}%",
        decision.expected_margin * 100.0
    );
    println!("  break-even quote: ${:.2}", decision.break_even_quote);
    println!(
        "  risk-adjusted quote: ${:.2}",
        decision.risk_adjusted_quote
    );
    println!();
    println!("Risk");
    println!(
        "  probability of loss: {:.2}%",
        decision.loss_probability * 100.0
    );
    println!("  worst simulated loss: ${:.2}", decision.worst_loss);
    println!(
        "  probability of cash shortfall: {:.2}%",
        decision.cash_shortfall_probability * 100.0
    );
    println!(
        "  average cash shortfall: ${:.2}",
        decision.average_cash_shortfall
    );
    println!("  max cash shortfall: ${:.2}", decision.max_cash_shortfall);
    println!();
    println!("Agent consequence");
    println!("  survival delta: ${:.2}", decision.survival_delta);
    println!("  control surface: {:.2}", decision.control_surface);
}

fn control_surface(average_profit: f64, average_cash_shortfall: f64) -> f64 {
    average_profit / (1.0 + average_cash_shortfall)
}
