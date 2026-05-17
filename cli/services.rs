pub mod cmd;
pub mod env;
pub mod kube;

pub type CommandResult<T = ()> = anyhow::Result<T>;
pub type CommandOutput<T> = CommandResult<T>;
