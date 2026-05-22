pub mod cmd;
pub mod creds;
pub mod env;
pub mod kube;
pub mod machine;
pub mod display;

pub type CommandResult<T = ()> = anyhow::Result<T>;
pub type CommandOutput<T> = CommandResult<T>;
