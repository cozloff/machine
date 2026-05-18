use rand::thread_rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Deserialize;
use std::fs;
use std::time::{Duration, Instant};

#[derive(Debug, Deserialize)]
struct Config {
    initial_stock_price: f64,
    shares_pledged: f64,
    loan_amount: f64,
    minimum_collateral_ratio: f64,
    cash_on_hand: f64,
    annual_volatility: f64,
    annual_drift: f64,
    days: usize,
    simulations: usize,
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
    let initial_stock_price = config.initial_stock_price;
    let shares_pledged = config.shares_pledged;
    let loan_amount = config.loan_amount;
    let minimum_collateral_ratio = config.minimum_collateral_ratio;
    let _cash_on_hand = config.cash_on_hand;
    let annual_volatility = config.annual_volatility;
    let annual_drift = config.annual_drift;
    let days = config.days;
    let simulations = config.simulations;

    let dt = 1.0 / days as f64;
    let mut rng = thread_rng();
    let setup_elapsed = setup_started.elapsed();

    let simulation_started = Instant::now();
    let mut random_sampling_elapsed = Duration::ZERO;
    let mut price_update_elapsed = Duration::ZERO;
    let mut shortfall_calc_elapsed = Duration::ZERO;
    let mut simulation_aggregation_elapsed = Duration::ZERO;
    let mut simulations_with_shortfall = 0usize;
    let mut total_worst_shortfall = 0.0;
    let mut max_worst_shortfall: f64 = 0.0;

    for _ in 0..simulations {
        let mut stock_price = initial_stock_price;
        let mut worst_shortfall: f64 = 0.0;

        for _ in 0..days {
            let random_sampling_started = Instant::now();
            let random_return: f64 = StandardNormal.sample(&mut rng);
            random_sampling_elapsed += random_sampling_started.elapsed();

            let price_update_started = Instant::now();
            let variance_adjusted_drift =
                (annual_drift - 0.5 * annual_volatility.powi(2)) * dt;

            let random_shock =
                annual_volatility * dt.sqrt() * random_return;

            let daily_log_return =
                variance_adjusted_drift + random_shock;

            let daily_growth_multiplier =
                daily_log_return.exp();

            stock_price *= daily_growth_multiplier;
            price_update_elapsed += price_update_started.elapsed();

            let shortfall_calc_started = Instant::now();
            let collateral_value = shares_pledged * stock_price;
            let required_value = loan_amount * minimum_collateral_ratio;
            let shortfall = f64::max(0.0, required_value - collateral_value);

            worst_shortfall = f64::max(worst_shortfall, shortfall);
            shortfall_calc_elapsed += shortfall_calc_started.elapsed();
        }

        let simulation_aggregation_started = Instant::now();
        if worst_shortfall > 0.0 {
            simulations_with_shortfall += 1;
        }

        total_worst_shortfall += worst_shortfall;
        max_worst_shortfall = max_worst_shortfall.max(worst_shortfall);
        simulation_aggregation_elapsed += simulation_aggregation_started.elapsed();
    }

    let simulation_elapsed = simulation_started.elapsed();
    let report_started = Instant::now();

    let probability_of_shortfall = simulations_with_shortfall as f64 / simulations as f64;
    let average_worst_shortfall = total_worst_shortfall / simulations as f64;

    println!("Risk summary");
    println!("  simulations: {}", simulations);
    println!("  days per simulation: {}", days);
    println!("  probability of shortfall: {:.2}%", probability_of_shortfall * 100.0);
    println!("  average worst shortfall: ${:.2}", average_worst_shortfall);
    println!("  max worst shortfall: ${:.2}", max_worst_shortfall);
    println!();
    println!("Timing");
    print_timing("config load", config_elapsed);
    print_timing("setup", setup_elapsed);
    print_timing("simulation", simulation_elapsed);
    print_timing("  random sampling", random_sampling_elapsed);
    print_timing("  price update", price_update_elapsed);
    print_timing("  shortfall calc", shortfall_calc_elapsed);
    print_timing("  simulation aggregation", simulation_aggregation_elapsed);
    print_timing(
        "  timing overhead / unmeasured",
        simulation_elapsed
            .saturating_sub(random_sampling_elapsed)
            .saturating_sub(price_update_elapsed)
            .saturating_sub(shortfall_calc_elapsed)
            .saturating_sub(simulation_aggregation_elapsed),
    );
    print_timing("report", report_started.elapsed());
    print_timing("total", total_started.elapsed());
}

fn print_timing(label: &str, elapsed: Duration) {
    println!("  {label}: {:.3} ms", elapsed.as_secs_f64() * 1_000.0);
}
