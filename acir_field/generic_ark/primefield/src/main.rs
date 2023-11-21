#[macro_use]
extern crate honggfuzz;

use acir_field::FieldElement;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let fe = FieldElement::from_be_bytes_reduce(data);

            fe.is_zero();
            fe.is_one();
            let exponent = FieldElement::try_from_str("2").unwrap();
            fe.pow(&exponent);
            let exponent = FieldElement::try_from_str("3").unwrap();
            fe.pow(&exponent);
            fe.num_bits();
            fe.fits_in_u128();
            fe.to_u128();
            fe.try_into_u128();
            fe.try_to_u64();
            fe.inverse();
            fe.try_inverse();
            fe.to_hex();
            fe.to_be_bytes();
            fe.bits();
            fe.fetch_nearest_bytes(0);
            fe.fetch_nearest_bytes(1);
            fe.fetch_nearest_bytes(256);
            fe.fetch_nearest_bytes(128);

            let fe2 = FieldElement::try_from_str("67890").unwrap();
            fe.and(&fe2, 256);
            fe.and(&fe2, 128);
            fe.and(&fe2, 64);

            fe.xor(&fe2, 256);
            fe.xor(&fe2, 128);
            fe.xor(&fe2, 64);

        });
    }
}
