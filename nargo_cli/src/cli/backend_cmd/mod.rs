use clap::{Args, Subcommand};

use crate::errors::CliError;

pub mod backends;
pub mod current_cmd;
pub mod install_cmd;
pub mod ls_cmd;
pub mod uninstall_cmd;
pub mod use_cmd;

#[non_exhaustive]
#[derive(Args, Clone, Debug)]
/// Install and select custom backends used to generate and verify proofs.
pub struct BackendCommand {
    #[command(subcommand)]
    command: BackendCommands,
}

#[non_exhaustive]
#[derive(Subcommand, Clone, Debug)]
pub enum BackendCommands {
    Current(current_cmd::CurrentCommand),
    Ls(ls_cmd::LsCommand),
    Use(use_cmd::UseCommand),
    Install(install_cmd::InstallCommand),
    Uninstall(uninstall_cmd::UninstallCommand),
}

pub fn run(cmd: BackendCommand) -> Result<(), CliError> {
    let BackendCommand { command } = cmd;

    match command {
        BackendCommands::Current(args) => current_cmd::run(args),
        BackendCommands::Ls(args) => ls_cmd::run(args),
        BackendCommands::Use(args) => use_cmd::run(args),
        BackendCommands::Install(args) => install_cmd::run(args),
        BackendCommands::Uninstall(args) => uninstall_cmd::run(args),
    }?;

    Ok(())
}
