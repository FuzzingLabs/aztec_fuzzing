use clap::Args;

use super::backends::get_active_backend; // Added missing crate prefix

use crate::errors::{BackendError, CliError};

/// Prints the name of the currently active backend
#[derive(Debug, Clone, Args)]
pub struct CurrentCommand;

pub(crate) fn run(_args: CurrentCommand) -> Result<(), CliError> {
    println!("{}", get_active_backend());

    Ok(())
}
