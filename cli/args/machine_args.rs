use clap::{Args, Subcommand};

#[derive(Args)]
pub struct MachineArgs {
    #[command(subcommand)]
    pub command: MachineCommand,
}

#[derive(Subcommand)]
pub enum MachineCommand {
    /// Start up machine docker
    Up,
    /// Shut down machine docker
    Down,
    /// Rebuild machine docker
    Rebuild,
    /// Start Ingestion workflow
    Ingest,
    /// Start Rho-guesser
    Rho,
    /// Start gpu reporting
    Gpu,
    /// SSD Commands
    Ssd(SsdArgs),
    /// Parquet subcommands
    Parquet(ParquetArgs),
}

#[derive(Args)]
pub struct ParquetArgs {
    #[command(subcommand)]
    pub command: ParquetCommand,
}

#[derive(Args)]
pub struct SsdArgs {
    #[command(subcommand)]
    pub command: SsdCommand,
}

#[derive(Clone, Copy, Subcommand)]
pub enum SsdCommand {
    /// Get SSD information
    Info,
}

#[derive(Clone, Copy, Subcommand)]
pub enum ParquetCommand {
    /// Run parquet write test
    Create,
    /// Run parquet compression test
    Nvcomp,
}
