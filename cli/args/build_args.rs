use clap::{Args, Subcommand};

#[derive(Args)]
pub struct BuildArgs {
    #[command(subcommand)]
    pub command: Option<BuildCommand>,
}

#[derive(Subcommand)]
pub enum BuildCommand {
    /// Rebuild and install to PATH
    Up,
}
