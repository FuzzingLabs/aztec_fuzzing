#[macro_use]
extern crate honggfuzz;

use blake2::{Blake2s256, Digest};
use acir::FieldElement;

/// Does a generic hash of the entire inputs converting the resulting hash into a single output field.
fn generic_hash_to_field<D: Digest>(message: &[u8]) -> Result<FieldElement, String> {
    let output_bytes: [u8; 32] =
        D::digest(message).as_slice().try_into().map_err(|_| "digest should be 256 bits")?;

    Ok(FieldElement::from_be_bytes_reduce(&output_bytes))
}

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let _ = generic_hash_to_field::<Blake2s256>(data);
        });
    }
}