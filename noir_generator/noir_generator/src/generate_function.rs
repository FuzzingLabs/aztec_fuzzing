use crate::generate_instruction;
use crate::bloc_variables::BlocVariables;

pub fn generate_function(function_name: String) -> String{
    let mut function: String = format!("fn {}() {{\n", function_name);

    let bloc_variables: BlocVariables = BlocVariables::new();

    // TODO
    function = format!("{}{}", function, generate_instruction::generate_random_instruction(bloc_variables));
    
    format!("{}}}", function)
}