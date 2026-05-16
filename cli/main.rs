mod args;
mod commands;

use clap::Parser;
use args::Cli;

fn main() -> commands::CommandResult {
    let cli = Cli::parse();

    commands::run(cli)
}
