use crate::functions::list_functions::ListFunctions;
use crate::random::Random;
use crate::variables::bloc_variables::BlocVariables;
use crate::variables::list_structs::ListStructs;
use crate::variables::var_type;
use crate::instructions::type_instruction::generate_type_instruction;

pub fn generate_variable_declaration(random: &mut Random, bloc_variables: &mut BlocVariables, list_functions: &ListFunctions, list_structs: &ListStructs) -> String {

    let chosen_type = var_type::random_type(random, list_structs);

    let instr_string = generate_type_instruction(random, bloc_variables, list_functions, &chosen_type);

    let new_variable = bloc_variables.new_variable(&chosen_type, random.gen_bool());
    format!("{} = {};\n", new_variable.initialize(), instr_string)
}