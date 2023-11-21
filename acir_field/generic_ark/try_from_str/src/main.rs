#[macro_use]
extern crate honggfuzz;

use acir_field::FieldElement;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(str) = std::str::from_utf8(data) {
                let _ = FieldElement::try_from_str(str).unwrap();
            }
        });
    }
}