use clap::{Args, Subcommand};

#[derive(Args)]
pub struct CredsArgs {
    #[command(subcommand)]
    pub command: CredsCommand,
}

#[derive(Subcommand)]
pub enum CredsCommand {
    /// Create .env file with creds from az login
    Env,
}
