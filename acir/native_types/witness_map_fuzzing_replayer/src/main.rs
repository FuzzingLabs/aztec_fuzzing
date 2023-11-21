use acir::native_types::{WitnessMap, Witness};
use acir::FieldElement;
use std::fs::File;
use std::io::Read;
use std::any::Any;
use std::env;

fn _insert(witness_map: &mut WitnessMap, data: &[u8]) {
    let _ = witness_map.insert(
        Witness::new(u32::from_le_bytes([data[0], data[1], data[2], data[3]])),
        FieldElement::from(i128::from_le_bytes([
            data[4], data[5], data[6], data[7], data[8], data[9], data[10], data[11],
            data[12], data[13], data[14], data[15], data[16], data[17], data[18], data[19],
        ])),
    );
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

/*
fn main() {
    let mut witness_map = WitnessMap::new();

    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1]).unwrap();
    let mut contents = Vec::new();

    file.read_to_end(&mut contents).unwrap();

    let separator = b"##########";
    let mut byte_sequences = Vec::new();
    let mut current_sequence = Vec::new();
    let mut is_inside_sequence = false;

    for byte in &contents {
        if !is_inside_sequence {
            if current_sequence.ends_with(separator) {
                current_sequence.clear();
                is_inside_sequence = true;
            }
        }
        current_sequence.push(*byte);

        if current_sequence.ends_with(separator) {
            is_inside_sequence = false;
            current_sequence.truncate(current_sequence.len() - separator.len()); // Remove the separator
            byte_sequences.push(current_sequence.clone());
            current_sequence.clear();
        }
    }

    for sequence in &byte_sequences {
        println!("Sequence: {:?}", sequence);  // Affiche la séquence de bytes telle quelle

        let sequence = sequence.as_slice();
        if sequence.len() >= 20 {
            _insert(&mut witness_map, sequence);
        }
        if sequence.len() >= 4 {
            _get(&witness_map, sequence);
            _get_index(&witness_map, sequence);
            _contains_key(&witness_map, sequence);
        }
    }

}
*/

fn main() {
    let mut witness_map = WitnessMap::new();
    let mut file = File::open("halfempty.out").unwrap();
    let mut contents = Vec::new();

    file.read_to_end(&mut contents).unwrap();

    let separator = b"##########";
    let mut byte_sequences = Vec::new();
    let mut current_sequence = Vec::new();
    let mut is_inside_sequence = false;

    for byte in &contents {
        if !is_inside_sequence {
            if current_sequence.ends_with(separator) {
                current_sequence.clear();
                is_inside_sequence = true;
            }
        }
        current_sequence.push(*byte);

        if current_sequence.ends_with(separator) {
            is_inside_sequence = false;
            current_sequence.truncate(current_sequence.len() - separator.len()); // Remove the separator
            byte_sequences.push(current_sequence.clone());
            current_sequence.clear();
        }
    }

    for sequence in &byte_sequences {
        println!("Sequence: {:?}", sequence);  // Affiche la séquence de bytes telle quelle

        let sequence = sequence.as_slice();
        if sequence.len() >= 20 {
            _insert(&mut witness_map, sequence);
        }
        if sequence.len() >= 4 {
            _get(&witness_map, sequence);
            _get_index(&witness_map, sequence);
            _contains_key(&witness_map, sequence);
        }
    }

}