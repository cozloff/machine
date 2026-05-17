use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gum")]
#[command(version, about = "Run gum-powered terminal workflows")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run test workflows and terminal output examples
    Test(TestArgs),
    /// Run machine workflows
    Mach(MachineArgs),
    /// Run Kubernetes workflows
    Kube(KubernetesArgs),
}

#[derive(Args)]
pub struct TestArgs {
    #[command(subcommand)]
    pub command: Option<TestCommand>,
}

#[derive(Clone, Copy, Subcommand)]
pub enum TestCommand {
    /// Print the gumball machine test screen
    Art,
    /// Run test command one
    One,
    /// Run test command two
    Two,
    /// Run test command three
    Three,
    /// Run test command four
    Four,
    /// Run test command five
    Five,
    /// Run test command six
    Six,
    /// Run test command seven
    Seven,
    /// Run test command eight
    Eight,
    /// Run test command nine
    Nine,
    /// Run test command ten
    Ten,
}

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
    /// Parquet subcommands
    Parquet(ParquetArgs),
}

#[derive(Args)]
pub struct ParquetArgs {
    #[command(subcommand)]
    pub command: ParquetCommand,
}

#[derive(Clone, Copy, Subcommand)]
pub enum ParquetCommand {
    /// Run parquet write test
    Create,
    /// Run parquet compression test
    Nvcomp,
}

#[derive(Args)]
pub struct KubernetesArgs {
    #[command(subcommand)]
    pub command: KubernetesCommand,
}

#[derive(Subcommand)]
pub enum KubernetesCommand {
    /// Start up minikube and deploy machine
    Mini,
}
