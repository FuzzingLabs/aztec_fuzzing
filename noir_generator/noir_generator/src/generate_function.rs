use crate::generate_instruction;
use crate::bloc_variables::BlocVariables;
use crate::random;

pub fn generate_function(function_name: String) -> String{
    let mut function: String = format!("fn {}() {{\n", function_name);

    let mut bloc_variables = BlocVariables::new();

    let n = random::generate_random_number(0, 10);
    for _ in 0..n {
        function = format!("{}{}", function, generate_instruction::generate_random_instruction(&mut bloc_variables));     
    }
    
    format!("{}}}", function)
}