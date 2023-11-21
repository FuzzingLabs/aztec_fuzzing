#[macro_use]
extern crate honggfuzz;

use acir::native_types::{WitnessMap, Witness};
use acir::FieldElement;

use std::fs::OpenOptions;
use std::io::{Write, Result};

fn _insert(witness_map: &mut WitnessMap, data: &[u8]) {
    let _ = witness_map.insert(Witness::new(u32::from_le_bytes([data[0], data[1], data[2], data[3]])), FieldElement::from(i128::from_le_bytes([data[4], data[5], data[6], data[7], data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15], data[16], data[17], data[18], data[19]])));
}

fn _get(witness_map: &WitnessMap, data: &[u8]) {
    let _ = witness_map.get(&Witness::new(u32::from_le_bytes([data[0], data[1], data[2], data[3]])));
}

fn _get_index(witness_map: &WitnessMap, data: &[u8]) {
    let _ = witness_map.get_index(u32::from_le_bytes([data[0], data[1], data[2], data[3]]));
}

fn _contains_key(witness_map: &WitnessMap, data: &[u8]) {
    let _ = witness_map.contains_key(&Witness::new(u32::from_le_bytes([data[0], data[1], data[2], data[3]])));
}

fn main() {
    let mut witness_map = WitnessMap::new();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .unwrap();
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() >= 4 {
                file.write_all(&[data, b"##########"].concat()).unwrap();
            }
            if data.len() >= 20 {
                _insert(&mut witness_map, data);
            }
            if data.len() >= 4 {
                _get(&witness_map, data);
                _get_index(&witness_map, data);
                _contains_key(&witness_map, data);
            }
        });
    }
}
