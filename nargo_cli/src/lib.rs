use std::path::Path;

use backend_interface::Backend;
use noirc_driver::CompileOptions;
pub mod cli;
pub mod errors;
pub mod fuzzinglabs;
use crate::cli::{compile_cmd::CompileCommand, NargoConfig};
use crate::errors::CliError;
pub fn fuzzinglabs_run() -> Result<(), CliError> {
    let backend = Backend::new("acvm-backend-barretenberg".to_string());
    let args = CompileCommand {
        package: None,
        workspace: false,
        compile_options: CompileOptions {
            force_compile: false,
            show_ssa: false,
            show_brillig: false,
            print_acir: false,
            deny_warnings: false,
            silence_warnings: false,
            only_acir: false,
            disable_macros: false,
        },
    };
    let config = NargoConfig {
        program_dir: Path::new("/home/raefko/aztec_fuzzing/noir_generator/noir_project")
            .to_path_buf(),
    };
    return cli::compile_cmd::run(&backend, args, config);
}
