use acir_field::FieldElement;
use brillig::Value;
use brillig::RegisterIndex;
use brillig_vm::Registers;

fn main() {
    /*
    println!("\n####################################################\n");
    println!("Inverse:\n--------\n");
    
    let fe = FieldElement::try_from_str("2").unwrap();
    println!("fe: {:?}", fe);
    let fe2 = fe.inverse();
    println!("fe.inverse(): {:?}", fe2);
    let exponent = FieldElement::try_from_str("0").unwrap();
    println!("exponent: {:?}", exponent);
    let fe3 = fe2.pow(&exponent);
    println!("fe.pow(fe): {:?}", fe3);


    let field_element = FieldElement::try_from_str("10944121435919637611123202872628637544274182200208017171849102093287904247809").unwrap();
    println!("field_element: {:?}", field_element);
    let inversed_field_element = field_element.inverse();
    println!("inversed_field_element: {:?}", inversed_field_element);

    println!("----------------------");

    println!("\n####################################################\n");
    println!("Inverse Pow Inverse Pow:\n------------------------\n");

    let fe1 = FieldElement::try_from_str("100").unwrap();
    let fe2 = fe1.inverse();
    let fe3 = fe2.pow(&FieldElement::try_from_str("2").unwrap());
    let fe4 = fe3.inverse();

    println!("fe1: {:?}", fe1);
    println!("fe2: {:?}", fe2);
    println!("fe3: {:?}", fe3);
    println!("fe4: {:?}", fe4);

    println!("----------------------");
    println!("\n####################################################\n");
    */

    let v1 = Value::from(10u128);
    let v2 = Value::from(20u128);
    let v3 = Value::from(30u128);
    let v4 = Value::from(40u128);

    let vec = vec![v1, v2, v3, v4];

    let mut r1 = Registers::load(vec);
    println!("r1: {:?}", r1);

    // let ri1 = RegisterIndex::from(7);
    // max register index is 65535
    // usize range is from 0 to 18446744073709551615
    let ri1 = RegisterIndex::from(18446744073709551615);
    let v5 = Value::from(0u128);
    r1.set(ri1, v5);
    println!("r1: {:?}", r1);
    println!("r1 length: {:?}", r1.inner.len());
}
