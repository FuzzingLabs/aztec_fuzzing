#[macro_use]
extern crate honggfuzz;

use acir::native_types::{WitnessMap, Witness};
use acir::FieldElement;

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
    loop {
        fuzz!(|data: &[u8]| {
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

/*
fn main() {
    let mut witness_map = WitnessMap::new();

    let witness_1 = Witness::new(1);
    let witness_2 = Witness::new(2);
    let value_1 = FieldElement::try_from_str("42").unwrap();
    let value_2 = FieldElement::try_from_str("17").unwrap();

    // Insert values into the WitnessMap
    witness_map.insert(witness_1, value_1);
    witness_map.insert(witness_2, value_2);

    let get1 = witness_map.get(&witness_1);
    let get2 = witness_map.get(&witness_2);
    let get3 = witness_map.get(&Witness::new(3));

    // check that the values are correct

    assert!(get1.is_some());
    assert!(get2.is_some());
    assert!(get3.is_none());

    assert!(get1.unwrap() == &value_1);
    assert!(get2.unwrap() == &value_2);
    assert!(get3.is_none());

    let get_index1 = witness_map.get_index(0);
    let get_index2 = witness_map.get_index(1);
    let get_index3 = witness_map.get_index(2);

    // check that the values are correct

    assert!(get_index1.is_none());
    assert!(get_index2.is_some());
    assert!(get_index3.is_some());

    assert!(get_index1.is_none());
    assert!(get_index2.unwrap() == &value_1);
    assert!(get_index3.unwrap() == &value_2);

    let bool1 = witness_map.contains_key(&witness_1);
    let bool2 = witness_map.contains_key(&witness_2);
    let bool3 = witness_map.contains_key(&Witness::new(3));

    // check that the values are correct

    assert!(bool1);
    assert!(bool2);
    assert!(!bool3);

}
*/