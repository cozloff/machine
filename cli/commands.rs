use crate::args::{Cli, Commands};

pub mod machine;
pub mod test;

pub type CommandResult = Result<(), Box<dyn std::error::Error>>;

pub fn run(cli: Cli) -> CommandResult {
    match cli.command {
        Commands::Test(args) => test::run(args),
        Commands::Mach(args) => machine::run(args),
    }
}
