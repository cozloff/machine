use crate::args::machine_args::{
    MachineArgs, 
    MachineCommand, 
    ParquetArgs, 
    ParquetCommand,
    SsdArgs,
    SsdCommand
};
use crate::commands::CommandResult;
use crate::services::cmd::run as cmd;
use crate::services::machine::machine_inspect::inspect_and_display;

pub fn run(args: MachineArgs) -> CommandResult {
    match args.command {
        MachineCommand::Up => up(),
        MachineCommand::Down => down(),
        MachineCommand::Rebuild => rebuild(),
        MachineCommand::Ingest => ingest(),
        MachineCommand::Rho => rho(),
        MachineCommand::Gpu => gpu(),
        MachineCommand::Parquet(args) => parquet(args),
        MachineCommand::Ssd(args) => ssd(args),
    }
}

fn up() -> CommandResult {
    cmd("docker", &["compose", "up", "-d", "--build"])
}

fn down() -> CommandResult {
    cmd("docker", &["compose", "down"])
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

fn ssd(args: SsdArgs) -> CommandResult {
    match args.command {
        SsdCommand::Info => inspect_ssd(),
    }
}

fn inspect_ssd() -> CommandResult {
    inspect_and_display()?;
    Ok(())
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
    let mut cmd_args = vec!["./build/container/machine"];
    cmd_args.extend_from_slice(args);
    run_compose_exec(&cmd_args)
}

fn run_compose_exec(args: &[&str]) -> CommandResult {
    let mut cmd_args = vec!["compose", "exec", "machine-dev"];
    cmd_args.extend_from_slice(args);
    cmd("docker", &cmd_args)
}
