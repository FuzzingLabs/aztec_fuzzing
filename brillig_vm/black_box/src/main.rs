#[macro_use]
extern crate honggfuzz;

use acir::brillig::Value;


fn to_value_vec(input: &[u8]) -> Vec<Value> {
    input.iter().map(|x| Value::from(*x as usize)).collect()
}

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let _ = to_value_vec(data);
        });
    }
}
