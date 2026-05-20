use crate::args::creds_args::{CredsArgs, CredsCommand};
use crate::commands::CommandResult;
use crate::services::cmd::ProcessCmd;
use crate::services::creds::keyvault::{Creds, CredsCli};

pub fn run(args: CredsArgs) -> CommandResult {
    let creds = CredsCli::new(ProcessCmd);

    match args.command {
        CredsCommand::Env => creds.get_env(),
    }
}
