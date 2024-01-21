use crate::constants::NB_MAX_INSTRUCTION;
use crate::statements::random_statement;
use crate::variables::bloc_variables::BlocVariables;
use crate::random;

pub fn generate_function(function_name: String) -> String{
    let mut function: String = format!("fn {}() {{\n", function_name);

    let mut bloc_variables = BlocVariables::new();

    for _ in 0..random::gen_range(0, NB_MAX_INSTRUCTION) {
        function = format!("{}{}", function, random_statement::generate_random_statement(&mut bloc_variables));     
    }
    
    format!("{}}}", function)
}