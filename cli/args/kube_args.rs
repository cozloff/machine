use clap::{Args, Subcommand};

#[derive(Args)]
pub struct KubernetesArgs {
    #[command(subcommand)]
    pub command: KubernetesCommand,
}

#[derive(Subcommand)]
pub enum KubernetesCommand {
    /// Start up minikube and deploy machine
    Mini,
    /// Deploy subcommands
    Deploy(DeployArgs),
}

#[derive(Args)]
pub struct DeployArgs {
    #[command(subcommand)]
    pub command: DeployCommand,
}

#[derive(Subcommand)]
pub enum DeployCommand {
    /// Machine subcommands
    Machine(DeployMachineArgs),
}

#[derive(Args)]
pub struct DeployMachineArgs {
    #[command(subcommand)]
    pub command: MachineTargets,
}

#[derive(Subcommand)]
pub enum MachineTargets {
    /// Machine to local machine cluster
    Local,
}
