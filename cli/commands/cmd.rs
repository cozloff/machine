use crate::commands::{CommandOutput, CommandResult};
use std::io;
use std::process::Command;

pub trait Cmd {
    fn run(&self, program: &str, args: &[&str]) -> CommandResult;
    fn capture(&self, program: &str, args: &[&str]) -> CommandOutput<String>;
    fn ok(&self, program: &str, args: &[&str]) -> bool;
}

pub struct ProcessCmd;

impl Cmd for ProcessCmd {
    fn run(&self, program: &str, args: &[&str]) -> CommandResult {
        let status = Command::new(program).args(args).status()?;

        if status.success() {
            return Ok(());
        }

        Err(command_error(program, args, status).into())
    }

    fn capture(&self, program: &str, args: &[&str]) -> CommandOutput<String> {
        let output = Command::new(program).args(args).output()?;

        if output.status.success() {
            return Ok(String::from_utf8(output.stdout)?);
        }

        Err(command_error(program, args, output.status).into())
    }

    fn ok(&self, program: &str, args: &[&str]) -> bool {
        Command::new(program)
            .args(args)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

fn command_error(program: &str, args: &[&str], status: std::process::ExitStatus) -> io::Error {
    let command = std::iter::once(program)
        .chain(args.iter().copied())
        .collect::<Vec<_>>()
        .join(" ");

    io::Error::new(
        io::ErrorKind::Other,
        format!("command failed with status {status}: {command}"),
    )
}