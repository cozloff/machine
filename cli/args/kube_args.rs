use clap::{Args, Subcommand};

#[derive(Args)]
pub struct KubernetesArgs {
    #[command(subcommand)]
    pub command: KubernetesCommand,
}

#[derive(Subcommand)]
pub enum KubernetesCommand {
    /// Configure the local k3s cluster
    K3s,
    /// Deploy subcommands
    Deploy(DeployArgs),
    /// Port-forward local services
    Forward(ForwardArgs),
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

#[derive(Args)]
pub struct ForwardArgs {
    #[command(subcommand)]
    pub command: ForwardCommand,
}

#[derive(Subcommand)]
pub enum ForwardCommand {
    /// Forward Grafana to localhost
    Grafana(PortForwardArgs),
}

#[derive(Args)]
pub struct PortForwardArgs {
    /// Local port to listen on
    #[arg(long, default_value_t = 3000)]
    pub local_port: u16,
}
