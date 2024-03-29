use std::path::PathBuf;

use backend_interface::backends_directory;
pub use backend_interface::Backend;

fn active_backend_file_path() -> PathBuf {
    backends_directory().join(".selected_backend")
}

pub use backend_interface::ACVM_BACKEND_BARRETENBERG;

pub fn clear_active_backend() {
    let active_backend_file = active_backend_file_path();
    if active_backend_file.is_file() {
        std::fs::remove_file(active_backend_file_path())
            .expect("should delete active backend file");
    }
}

pub fn set_active_backend(backend_name: &str) {
    let active_backend_file = active_backend_file_path();
    let backends_directory =
        active_backend_file.parent().expect("active backend file should have parent");

    std::fs::create_dir_all(backends_directory).expect("Could not create backends directory");
    std::fs::write(active_backend_file, backend_name.as_bytes())
        .expect("Could not write to active backend file");
}

pub fn get_active_backend() -> String {
    let active_backend_file = active_backend_file_path();

    if !active_backend_file.is_file() {
        set_active_backend(ACVM_BACKEND_BARRETENBERG);
        return ACVM_BACKEND_BARRETENBERG.to_string();
    }

    std::fs::read_to_string(active_backend_file).expect("Could not read active backend file")
}
