mod kernel;

use kernel::action::ActionTicket;
use kernel::business::BusinessOrder;
use kernel::execution::{StageTiming, print_timing_report};
use kernel::information::load_request;
use kernel::integration::IntegrationPlan;
use kernel::objective::GoalFunction;
use kernel::policy::{
    RiskEngine, print_action_ticket, print_business_order, print_goal_function,
    print_reality_signal, print_report,
};
use kernel::product::ProductLoop;
use kernel::reality::RealityObservation;
use std::env;
use std::time::Instant;

fn main() {
    let total_started = Instant::now();
    let mut timing = StageTiming::default();
    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("config.toml"));

    let config_started = Instant::now();
    let request = load_request(&config_path);
    timing.config_load = config_started.elapsed();

    let setup_started = Instant::now();
    let engine = RiskEngine::new(request);
    timing.setup = setup_started.elapsed();

    let trace = engine.simulate(&mut timing);
    let decision = engine.decide(&trace);
    let observation = engine
        .request()
        .reality_observation
        .as_ref()
        .map(|input| RealityObservation::from_input(input, engine.request()));
    let reality_signal = engine.observe_reality(&decision, observation);
    let goal = GoalFunction::agent_1_reality_optimization();
    let reward = goal.score(&decision, &reality_signal);
    let product_loop = ProductLoop::quote_risk_engine();
    let integrations = IntegrationPlan::from_request(engine.request());
    let action_ticket = ActionTicket::from_state(
        engine.request(),
        &decision,
        &reality_signal,
        &product_loop,
        &integrations,
    );
    let business_order = BusinessOrder::from_state(&action_ticket, &reality_signal, &reward);

    let report_started = Instant::now();
    print_report(engine.agent(), engine.request(), &decision);
    print_reality_signal(&reality_signal);
    print_goal_function(&goal, &reward, &product_loop);
    print_action_ticket(&action_ticket);
    print_business_order(&business_order);
    timing.report = report_started.elapsed();

    action_ticket
        .write()
        .expect("failed to write next action ticket");
    business_order
        .write()
        .expect("failed to write business order");
    integrations
        .write(engine.request(), &action_ticket)
        .expect("failed to write integration payload");

    timing.total = total_started.elapsed();
    print_timing_report(&timing);
}
