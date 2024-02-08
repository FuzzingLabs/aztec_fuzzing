use clap::Args;

use super::backends::set_active_backend;
use crate::errors::{BackendError, CliError};

use super::ls_cmd::get_available_backends;

/// Select the backend to use
#[derive(Debug, Clone, Args)]
pub struct UseCommand {
    backend: String,
}

pub fn run(args: UseCommand) -> Result<(), CliError> {
    let backends = get_available_backends();

    if !backends.contains(&args.backend) {
        return Err(BackendError::UnknownBackend(args.backend).into());
    }

    set_active_backend(&args.backend);

    Ok(())
}
