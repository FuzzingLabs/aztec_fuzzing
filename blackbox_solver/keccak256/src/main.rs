#[macro_use]
extern crate honggfuzz;

use acvm_blackbox_solver::keccak256;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let _ = keccak256(data).unwrap();
        });
    }
}