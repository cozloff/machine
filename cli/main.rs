mod args;
mod commands;
mod services;

use args::Cli;
use clap::Parser;

fn main() -> commands::CommandResult {
    services::env::load_dotenv()?;

    let cli = Cli::parse();

    commands::run(cli)
}
