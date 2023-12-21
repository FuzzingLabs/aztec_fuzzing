use crate::generate_instruction;
use crate::bloc_variables::BlocVariables;

pub fn generate_function(function_name: String) -> String{
    let mut function: String = format!("fn {}() {{\n", function_name);

    let mut bloc_variables = BlocVariables::new();

    // TODO
    for _ in 0..10 {
        function = format!("{}{}", function, generate_instruction::generate_random_instruction(&mut bloc_variables));     
    }
    
    format!("{}}}", function)
}