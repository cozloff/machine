use clap::{Parser, Subcommand};

pub mod test_args;
pub mod machine_args;
pub mod kube_args;
pub mod creds_args;

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
    Test(test_args::TestArgs),
    /// Run machine workflows
    Mach(machine_args::MachineArgs),
    /// Run Kubernetes workflows
    Kube(kube_args::KubernetesArgs),
    /// Run credential management workflows
    Creds(creds_args::CredsArgs),
}