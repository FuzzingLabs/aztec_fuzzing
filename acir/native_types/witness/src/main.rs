#[macro_use]
extern crate honggfuzz;

use acir::native_types::Witness;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() != 4 {
                return;
            }
            let witness_index: u32 = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            /*
            let witness1 = Witness::new(witness_index);
            let _ = witness1.as_usize();
            let witness2 = Witness::from(witness_index);
            assert!(witness1 == witness2);
            */
            
            
            // println!("witness_index: {:?}", witness_index);
            let witness1 = Witness::new(witness_index);
            let u_size = witness1.as_usize();
            // println!("u_size: {:?}", u_size);
            let witness2 = Witness::from(witness_index);
            // println!("witness1: {:?}", witness1);
            // println!("witness2: {:?}", witness2);
            assert!(witness1 == witness2);
            let expression_for_witness3 = witness1 + witness2;
            let witness3 = expression_for_witness3.linear_combinations[0].0.try_to_u64().unwrap() as u32 * expression_for_witness3.linear_combinations[0].1.witness_index();
            // println!("witness3: {:?}", witness3);
            assert!(witness3 == witness2.witness_index() + witness1.witness_index());
            
        });
    }
}
