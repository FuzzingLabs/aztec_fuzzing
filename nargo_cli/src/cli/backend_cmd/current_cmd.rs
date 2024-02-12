use clap::Args;

use crate::{backends::get_active_backend, errors::CliError};

/// Prints the name of the currently active backend
#[derive(Debug, Clone, Args)]
pub struct CurrentCommand;

pub fn run(_args: CurrentCommand) -> Result<(), CliError> {
    println!("{}", get_active_backend());

    Ok(())
}
