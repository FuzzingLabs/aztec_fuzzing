#[macro_use]
extern crate honggfuzz;

use acir_field::FieldElement;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let hex_test: String = data.iter()
                .map(|byte| format!("{:02X}", byte))
                .collect();
            let _ = FieldElement::try_from_str(&hex_test).unwrap();
        });
        
    }
}