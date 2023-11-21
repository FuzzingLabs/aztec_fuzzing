use acir_field::FieldElement;
use brillig::Value;
use brillig_vm::Registers;
use brillig::RegisterIndex;
use std::panic::catch_unwind;
extern crate num_bigint;

fn main() {
    println!("\n####################################################\n");


    println!("Problem n°1\n-----------\n");
    
    let field_element = FieldElement::try_from_str("340282366920938463463374607431768211455").unwrap();
    println!("field_element: {:?}", field_element);
    
    let to_u128 = field_element.to_u128();
    println!("to_u128: {}", to_u128);

    let result = catch_unwind(|| field_element.try_into_u128());
    if let Ok(try_into_u128) = result {
        println!("try_into_u128: {:?}", try_into_u128);
    } else if let Err(panic_payload) = result {
        panic!("Panic occurred during try_into_u128: {:?}", panic_payload);
    }

    println!("----------------------");

    let field_element = FieldElement::try_from_str("340282366920938463463374607431768211456").unwrap();
    println!("field_element: {:?}", field_element);

    let to_u128 = field_element.to_u128();
    println!("to_u128: {}", to_u128);

    let result = catch_unwind(|| field_element.try_into_u128());
    if let Ok(try_into_u128) = result {
        println!("try_into_u128: {:?}", try_into_u128);
    } else if let Err(panic_payload) = result {
        panic!("Panic occurred during try_into_u128: {:?}", panic_payload);
    }

    println!("----------------------");


    let field_element = FieldElement::try_from_str("340282366920938463463374607431768211457").unwrap();
    println!("field_element: {:?}", field_element);
    let to_u128 = field_element.to_u128();
    println!("to_u128: {}", to_u128);
    let result = catch_unwind(|| field_element.try_into_u128());
    if let Ok(try_into_u128) = result {
        println!("try_into_u128: {:?}", try_into_u128);
    } else if let Err(panic_payload) = result {
        panic!("Panic occurred during try_into_u128: {:?}", panic_payload);
    }

    /*
    let fe = FieldElement::from_be_bytes_reduce(&[255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]);
    let fe = FieldElement::from_be_bytes_reduce(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    */
    

    println!("\n####################################################\n");


    println!("Problem n°2\n-----------\n");


    let fe = FieldElement::try_from_str("10").unwrap();
    let exponent = FieldElement::try_from_str("3").unwrap();
    let result = fe.pow(&exponent);
    println!("field_element: {:?}", fe);
    println!("exponent: {:?}", exponent);
    println!("result: {:?}", result);

    println!("----------------------");

    let fe = FieldElement::try_from_str("340282366920938463463374607431768211455").unwrap();
    let exponent = FieldElement::try_from_str("340282366920938463463374607431768211455").unwrap();
    let result = fe.pow(&exponent);
    println!("field_element: {:?}", fe);
    println!("exponent: {:?}", exponent);
    println!("result: {:?}", result);
    let result = result.to_u128();
    println!("result: {:?}", result);
    let result = result.to_be_bytes();
    println!("result: {:?}", result);
    let nb : u128 = result.iter().fold(0, |acc, &x| acc * 256 + x as u128);
    println!("nb: {:?}", nb);


    println!("\n####################################################\n");
    *//*
    let fe = FieldElement::try_from_str("99999999").unwrap();
    println!("fe: {:?}", fe);
    let fe2 = fe.pow(&FieldElement::try_from_str("3").unwrap());
    println!("fe.pow(fe): {:?}", fe2);
    let fe3 = fe2.inverse();
    println!("fe2.inverse(): {:?}", fe3);
    let fe4 = fe3.pow(&FieldElement::try_from_str("3").unwrap());
    println!("fe3.pow(fe): {:?}", fe4);
    let fe5 = fe4.inverse();
    println!("fe4.inverse(): {:?}", fe5);

    println!("----------------------");

    let fe = FieldElement::try_from_str("99999999").unwrap();
    println!("fe: {:?}", fe);
    let fe2 = fe.pow(&FieldElement::try_from_str("9").unwrap());
    println!("fe.pow(fe): {:?}", fe2);
    */
    


    // max register index is 65535
    // usize range is from 0 to 18446744073709551615
    /*
    Registers::load(vec![]).get(RegisterIndex::from(65536));
    Registers::load(vec![]).set(RegisterIndex::from(65536), Value::from(0u128));
    */
    // The problem is inside the get & set methods of the Registers struct
    // The index is too big and it makes the asserts fail


    // println!("\n####################################################\n");
}
