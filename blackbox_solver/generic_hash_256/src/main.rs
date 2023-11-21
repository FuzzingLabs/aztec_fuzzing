#[macro_use]
extern crate honggfuzz;

use blake2::{Blake2s256, Digest};
use sha2::Sha256;
use sha3::Keccak256;

/// Does a generic hash of the inputs returning the resulting 32 bytes separately.
fn generic_hash_256<D: Digest>(message: &[u8]) -> Result<[u8; 32], String> {
    let output_bytes: [u8; 32] =
        D::digest(message).as_slice().try_into().map_err(|_| "digest should be 256 bits")?;

    Ok(output_bytes)
}

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let _ = generic_hash_256::<Sha256>(data);
            let _ = generic_hash_256::<Blake2s256>(data);
            let _ = generic_hash_256::<Keccak256>(data);
        });
    }
}