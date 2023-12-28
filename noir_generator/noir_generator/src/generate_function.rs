use crate::instructions::generate_instruction;
use crate::variables::bloc_variables::BlocVariables;
use crate::random;

pub fn generate_function(function_name: String) -> String{
    let mut function: String = format!("fn {}() {{\n", function_name);

    let mut bloc_variables = BlocVariables::new();

    let n = random::generate_random_number(0, 100);
    for _ in 0..n {
        function = format!("{}{}", function, generate_instruction::generate_random_instruction(&mut bloc_variables));     
    }
    
    format!("{}}}", function)
}