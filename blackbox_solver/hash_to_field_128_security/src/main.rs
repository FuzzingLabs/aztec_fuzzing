#[macro_use]
extern crate honggfuzz;

use acvm_blackbox_solver::hash_to_field_128_security;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let _ = hash_to_field_128_security(data).unwrap();
        });
    }
}