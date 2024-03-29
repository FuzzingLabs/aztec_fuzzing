use clap::Args;

use backend_interface::backends_directory;

use crate::{
    backends::{
        clear_active_backend, get_active_backend, set_active_backend, ACVM_BACKEND_BARRETENBERG,
    },
    errors::{BackendError, CliError},
};

use super::ls_cmd::get_available_backends;

/// Uninstalls a backend
#[derive(Debug, Clone, Args)]
pub struct UninstallCommand {
    /// The name of the backend to uninstall.
    backend: String,
}

pub fn run(args: UninstallCommand) -> Result<(), CliError> {
    let installed_backends = get_available_backends();

    if !installed_backends.contains(&args.backend) {
        return Err(BackendError::UnknownBackend(args.backend).into());
    }

    let active_backend = get_active_backend();

    // Handle the case where we're uninstalling the currently active backend.
    if active_backend == args.backend {
        let barretenberg_is_installed =
            installed_backends.iter().any(|backend_name| backend_name == ACVM_BACKEND_BARRETENBERG);

        let new_active_backend =
            if args.backend != ACVM_BACKEND_BARRETENBERG && barretenberg_is_installed {
                // Prefer switching to barretenberg if possible.
                Some(ACVM_BACKEND_BARRETENBERG)
            } else {
                // Otherwise pick the first backend which isn't being uninstalled.
                installed_backends
                    .iter()
                    .find(|&backend_name| backend_name != &args.backend)
                    .map(|name| name.as_str())
            };

        if let Some(backend) = new_active_backend {
            set_active_backend(backend);
        } else {
            // We've deleted the last backend. Clear the active backend file to be recreated once we install a new one.
            clear_active_backend();
        }
    }

    std::fs::remove_dir_all(backends_directory().join(args.backend))
        .expect("backend directory should be deleted");

    Ok(())
}
