#[macro_use]
extern crate honggfuzz;

use acvm_blackbox_solver::blake2s;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let _ = blake2s(data).unwrap();
        });
    }
}