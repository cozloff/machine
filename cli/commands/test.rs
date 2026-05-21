use crate::args::test_args::{TestArgs, TestCommand};
use crate::commands::CommandResult;
use colored::Colorize;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

const GUM_ART: &str = include_str!("../art/gum.txt");

pub fn run(args: TestArgs) -> CommandResult {
    match args.command.unwrap_or(TestCommand::Art) {
        TestCommand::Art => art(),
        TestCommand::One => numbered(1, "checking command wiring"),
        TestCommand::Two => numbered(2, "loading config shape"),
        TestCommand::Three => numbered(3, "validating terminal output"),
        TestCommand::Four => numbered(4, "testing nested command dispatch"),
        TestCommand::Five => numbered(5, "running workflow placeholder"),
        TestCommand::Six => numbered(6, "checking styled output"),
        TestCommand::Seven => numbered(7, "testing progress feedback"),
        TestCommand::Eight => numbered(8, "verifying command module boundaries"),
        TestCommand::Nine => numbered(9, "checking future expansion point"),
        TestCommand::Ten => numbered(10, "finishing test command set"),
    }
}

fn art() -> CommandResult {
    println!("{GUM_ART}");
    println!("{}", style("gum test complete").green().bold());
    println!("{}", "kinda ready for the next command".bright_black());
    Ok(())
}

fn numbered(number: u8, message: &str) -> CommandResult {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .expect("spinner template should be valid"),
    );
    spinner.set_message(message.to_owned());
    spinner.enable_steady_tick(Duration::from_millis(80));
    std::thread::sleep(Duration::from_millis(250));
    spinner.finish_and_clear();

    println!(
        "{} {}",
        style(format!("test {number}")).cyan().bold(),
        message.bright_black()
    );

    Ok(())
}
