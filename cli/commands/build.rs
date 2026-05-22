use crate::args::build_args::{BuildArgs, BuildCommand};
use crate::commands::CommandResult;
use crate::services::cmd::run as cmd;
use anyhow::Context;

const JUMPBOX_REPO_PATH: &str = "/workspace";

pub fn run(args: BuildArgs) -> CommandResult {
    match args.command.unwrap_or(BuildCommand::Up) {
        BuildCommand::Up => up(),
    }
}

fn up() -> CommandResult {
    cmd(
        "cargo",
        &["install", "--path", JUMPBOX_REPO_PATH, "--force"],
    )
    .with_context(|| format!("install gum from {JUMPBOX_REPO_PATH}"))
}
