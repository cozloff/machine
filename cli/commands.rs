use crate::args::{Cli, Commands};

pub mod creds;
pub mod kube;
pub mod machine;
pub mod test;

pub use crate::services::CommandResult;

pub fn run(cli: Cli) -> CommandResult {
    match cli.command {
        Commands::Test(args) => test::run(args),
        Commands::Mach(args) => machine::run(args),
        Commands::Kube(args) => kube::run(args),
        Commands::Creds(args) => creds::run(args),
    }
}
