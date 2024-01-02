use crate::instructions::generate_instruction;
use crate::variables::bloc_variables::BlocVariables;
use crate::random;

const NB_MAX_INSTRUCTION: u32 = 100;

pub fn generate_function(function_name: String) -> String{
    let mut function: String = format!("fn {}() {{\n", function_name);

    let mut bloc_variables = BlocVariables::new();

    for _ in 0..random::gen_range(0, NB_MAX_INSTRUCTION) {
        function = format!("{}{}", function, generate_instruction::generate_random_instruction(&mut bloc_variables));     
    }
    
    format!("{}}}", function)
}