#[macro_use]
extern crate honggfuzz;

use acvm_blackbox_solver::sha256;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let _ = sha256(data).unwrap();
        });
    }
}