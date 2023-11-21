#[macro_use]
extern crate honggfuzz;

use acir_field::FieldElement;

use std::panic::{catch_unwind, UnwindSafe};

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let result = catch_unwind(|| FieldElement::from_be_bytes_reduce(data));
            if let Ok(field_element) = result {
            } else if let Err(panic_payload) = result {
                panic!("Panic occurred: {:?}", panic_payload);
            }
        });
    }
}
