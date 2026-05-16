use crate::args::{MachineArgs, MachineCommand, ParquetArgs, ParquetCommand};
use crate::commands::CommandResult;
use std::io;
use std::process::Command;

pub fn run(args: MachineArgs) -> CommandResult {
    match args.command {
        MachineCommand::Up => up(),
        MachineCommand::Down => down(),
        MachineCommand::Rebuild => rebuild(),
        MachineCommand::Ingest => ingest(),
        MachineCommand::Rho => rho(),
        MachineCommand::Gpu => gpu(),
        MachineCommand::Parquet(args) => parquet(args),
    }
}

fn up() -> CommandResult {
    run_command("docker", &["compose", "up", "-d", "--build"])
}

fn down() -> CommandResult {
    run_command("docker", &["compose", "down"])
}

fn rebuild() -> CommandResult {
    run_compose_exec(&[
        "sh",
        "-lc",
        "cmake -E rm -rf build/container && cmake -S . -B build/container && cmake --build build/container && ./build/container/machine",
    ])
}

fn ingest() -> CommandResult {
    run_container_machine(&["ingest"])
}

fn rho() -> CommandResult {
    run_container_machine(&["gpu", "rho-guess"])
}

fn gpu() -> CommandResult {
    run_container_machine(&["gpu", "report"])
}

fn parquet(args: ParquetArgs) -> CommandResult {
    match args.command {
        ParquetCommand::Create => parquet_create(),
        ParquetCommand::Nvcomp => parquet_nvcomp(),
    }
}

fn parquet_create() -> CommandResult {
    run_container_machine(&["data", "parquet", "create"])
}

fn parquet_nvcomp() -> CommandResult {
    run_container_machine(&["data", "parquet", "nvcomp"])
}

fn run_container_machine(args: &[&str]) -> CommandResult {
    let mut command_args = vec!["./build/container/machine"];
    command_args.extend_from_slice(args);
    run_compose_exec(&command_args)
}

fn run_compose_exec(args: &[&str]) -> CommandResult {
    let mut command_args = vec!["compose", "exec", "machine-dev"];
    command_args.extend_from_slice(args);
    run_command("docker", &command_args)
}

fn run_command(program: &str, args: &[&str]) -> CommandResult {
    let status = Command::new(program).args(args).status()?;

    if status.success() {
        return Ok(());
    }

    let command = std::iter::once(program)
        .chain(args.iter().copied())
        .collect::<Vec<_>>()
        .join(" ");
    Err(io::Error::new(
        io::ErrorKind::Other,
        format!("command failed with status {status}: {command}"),
    )
    .into())
}
