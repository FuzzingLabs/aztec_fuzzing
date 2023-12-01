use crate::variables::Variables;

#[macro_use]
extern crate lazy_static;

mod variable;
mod variables;
mod values;
mod operators;
mod statments;
mod unary_operation_functions;
mod binary_operation_functions;
mod arithmetic_operation;
mod affectation_operation;
mod beautifier;

fn main() {
    
    println!("fn main() {{");

    for i in 0..200 {
        println!("//__________________________________________________________________________________________\n//\n// Opération n°{} :\n//__________________\n", i);
        
        println!("{}", beautifier::beautify_statment(statments::generate_statment(), false));
    }

    println!("}}");
   
}
