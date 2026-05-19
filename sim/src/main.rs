use rand::{thread_rng, Rng};
use rand_distr::{Distribution, StandardNormal};
use serde::Deserialize;
use std::fs;
use std::time::{Duration, Instant};

#[derive(Debug, Deserialize)]
struct Config {
    job_name: String,
    material_name: String,
    quote_price: f64,
    units: f64,
    base_material_cost_per_unit: f64,
    annual_material_volatility: f64,
    annual_material_drift: f64,
    base_machine_hours: f64,
    machine_hour_rate: f64,
    machine_time_std_dev_pct: f64,
    setup_hours: f64,
    labor_hours: f64,
    labor_hour_rate: f64,
    tooling_cost: f64,
    inspection_cost: f64,
    scrap_probability: f64,
    rework_probability: f64,
    rework_cost: f64,
    deadline_penalty_probability: f64,
    deadline_penalty_cost: f64,
    financing_annual_rate: f64,
    cash_on_hand: f64,
    days_until_paid: usize,
    simulations: usize,
}

#[derive(Debug, Default)]
struct SimulationStats {
    simulations_with_loss: usize,
    simulations_with_cash_shortfall: usize,
    total_profit: f64,
    total_cost: f64,
    total_cost_squared: f64,
    total_cash_shortfall: f64,
    worst_loss: f64,
    max_cash_shortfall: f64,
}

fn load_config(path: &str) -> Config {
    let raw = fs::read_to_string(path).expect("failed to read config file");
    toml::from_str(&raw).expect("failed to parse config file")
}

fn main() {
    let total_started = Instant::now();
    let config_started = Instant::now();
    let config = load_config("config.toml");
    let config_elapsed = config_started.elapsed();

    let setup_started = Instant::now();
    validate_config(&config);
    let mut rng = thread_rng();
    let setup_elapsed = setup_started.elapsed();

    let simulation_started = Instant::now();
    let mut random_sampling_elapsed = Duration::ZERO;
    let mut cost_model_elapsed = Duration::ZERO;
    let mut risk_calc_elapsed = Duration::ZERO;
    let mut simulation_aggregation_elapsed = Duration::ZERO;
    let mut stats = SimulationStats::default();

    for _ in 0..config.simulations {
        let random_sampling_started = Instant::now();
        let material_shock: f64 = StandardNormal.sample(&mut rng);
        let machine_time_shock: f64 = StandardNormal.sample(&mut rng);
        let scrap_happened = rng.gen_bool(config.scrap_probability);
        let rework_happened = rng.gen_bool(config.rework_probability);
        let deadline_penalty_happened = rng.gen_bool(config.deadline_penalty_probability);
        random_sampling_elapsed += random_sampling_started.elapsed();

        let cost_model_started = Instant::now();
        let material_cost = simulated_material_cost(&config, material_shock);
        let machine_hours = simulated_machine_hours(&config, machine_time_shock);
        let machine_cost = (config.setup_hours + machine_hours) * config.machine_hour_rate;
        let labor_cost = (config.setup_hours + config.labor_hours) * config.labor_hour_rate;

        let baseline_cost = material_cost
            + machine_cost
            + labor_cost
            + config.tooling_cost
            + config.inspection_cost;

        let scrap_cost = if scrap_happened {
            material_cost + (machine_cost * 0.50) + (config.tooling_cost * 0.25)
        } else {
            0.0
        };

        let rework_cost = if rework_happened {
            config.rework_cost
        } else {
            0.0
        };

        let deadline_penalty = if deadline_penalty_happened {
            config.deadline_penalty_cost
        } else {
            0.0
        };

        let pre_financing_cost = baseline_cost + scrap_cost + rework_cost + deadline_penalty;
        let financing_cost = pre_financing_cost
            * config.financing_annual_rate
            * (config.days_until_paid as f64 / 365.0);
        let total_cost = pre_financing_cost + financing_cost;
        cost_model_elapsed += cost_model_started.elapsed();

        let risk_calc_started = Instant::now();
        let profit = config.quote_price - total_cost;
        let cash_shortfall = f64::max(0.0, total_cost - config.cash_on_hand);
        risk_calc_elapsed += risk_calc_started.elapsed();

        let simulation_aggregation_started = Instant::now();
        if profit < 0.0 {
            stats.simulations_with_loss += 1;
            stats.worst_loss = stats.worst_loss.max(-profit);
        }

        if cash_shortfall > 0.0 {
            stats.simulations_with_cash_shortfall += 1;
            stats.max_cash_shortfall = stats.max_cash_shortfall.max(cash_shortfall);
        }

        stats.total_profit += profit;
        stats.total_cost += total_cost;
        stats.total_cost_squared += total_cost.powi(2);
        stats.total_cash_shortfall += cash_shortfall;
        simulation_aggregation_elapsed += simulation_aggregation_started.elapsed();
    }

    let simulation_elapsed = simulation_started.elapsed();
    let report_started = Instant::now();
    print_report(&config, &stats);
    let report_elapsed = report_started.elapsed();

    println!();
    println!("Timing");
    print_timing("config load", config_elapsed);
    print_timing("setup", setup_elapsed);
    print_timing("simulation", simulation_elapsed);
    print_timing("  random sampling", random_sampling_elapsed);
    print_timing("  cost model", cost_model_elapsed);
    print_timing("  risk calc", risk_calc_elapsed);
    print_timing("  simulation aggregation", simulation_aggregation_elapsed);
    print_timing(
        "  timing overhead / unmeasured",
        simulation_elapsed
            .saturating_sub(random_sampling_elapsed)
            .saturating_sub(cost_model_elapsed)
            .saturating_sub(risk_calc_elapsed)
            .saturating_sub(simulation_aggregation_elapsed),
    );
    print_timing("report", report_elapsed);
    print_timing("total", total_started.elapsed());
}

fn simulated_material_cost(config: &Config, random_return: f64) -> f64 {
    let years = config.days_until_paid as f64 / 252.0;
    let variance_adjusted_drift =
        (config.annual_material_drift - 0.5 * config.annual_material_volatility.powi(2)) * years;
    let random_shock = config.annual_material_volatility * years.sqrt() * random_return;
    let material_price_multiplier = (variance_adjusted_drift + random_shock).exp();

    config.units * config.base_material_cost_per_unit * material_price_multiplier
}

fn simulated_machine_hours(config: &Config, random_return: f64) -> f64 {
    let multiplier = 1.0 + (config.machine_time_std_dev_pct * random_return);
    config.base_machine_hours * multiplier.max(0.10)
}

fn validate_config(config: &Config) {
    assert!(config.quote_price > 0.0, "quote_price must be positive");
    assert!(config.units > 0.0, "units must be positive");
    assert!(
        config.base_material_cost_per_unit >= 0.0,
        "base_material_cost_per_unit must be non-negative"
    );
    assert!(
        config.base_machine_hours >= 0.0,
        "base_machine_hours must be non-negative"
    );
    assert!(config.simulations > 0, "simulations must be positive");

    for (name, probability) in [
        ("scrap_probability", config.scrap_probability),
        ("rework_probability", config.rework_probability),
        (
            "deadline_penalty_probability",
            config.deadline_penalty_probability,
        ),
    ] {
        assert!(
            (0.0..=1.0).contains(&probability),
            "{name} must be between 0.0 and 1.0"
        );
    }
}

fn print_report(config: &Config, stats: &SimulationStats) {
    let simulations = config.simulations as f64;
    let average_profit = stats.total_profit / simulations;
    let average_cost = stats.total_cost / simulations;
    let cost_variance = (stats.total_cost_squared / simulations) - average_cost.powi(2);
    let cost_std_dev = cost_variance.max(0.0).sqrt();
    let loss_probability = stats.simulations_with_loss as f64 / simulations;
    let cash_shortfall_probability = stats.simulations_with_cash_shortfall as f64 / simulations;
    let average_cash_shortfall = stats.total_cash_shortfall / simulations;
    let recommended_quote = average_cost + (1.65 * cost_std_dev);
    let expected_margin = average_profit / config.quote_price;

    println!("Manufacturing quote risk summary");
    println!("  job: {}", config.job_name);
    println!("  material: {}", config.material_name);
    println!("  simulations: {}", config.simulations);
    println!("  days until paid: {}", config.days_until_paid);
    println!();
    println!("Quote economics");
    println!("  quoted price: ${:.2}", config.quote_price);
    println!("  expected cost: ${:.2}", average_cost);
    println!("  expected profit: ${:.2}", average_profit);
    println!("  expected margin: {:.2}%", expected_margin * 100.0);
    println!("  break-even quote: ${:.2}", average_cost);
    println!("  risk-adjusted quote: ${:.2}", recommended_quote);
    println!();
    println!("Risk");
    println!("  probability of loss: {:.2}%", loss_probability * 100.0);
    println!("  worst simulated loss: ${:.2}", stats.worst_loss);
    println!(
        "  probability of cash shortfall: {:.2}%",
        cash_shortfall_probability * 100.0
    );
    println!("  average cash shortfall: ${:.2}", average_cash_shortfall);
    println!("  max cash shortfall: ${:.2}", stats.max_cash_shortfall);
}

fn print_timing(label: &str, elapsed: Duration) {
    println!("  {label}: {:.3} ms", elapsed.as_secs_f64() * 1_000.0);
}
