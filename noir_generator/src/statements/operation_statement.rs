use crate::functions::list_functions::ListFunctions;
use crate::random::Random;
use crate::variables::list_structs::ListStructs;
use crate::variables::variable::Variable;
use crate::variables::{bloc_data::BlocData, var_type};
use crate::instructions::type_instruction::generate_type_instruction;

pub fn generate_operation_instruction(random: &mut Random, bloc_variables: &mut BlocData, list_functions: &ListFunctions, list_structs: &ListStructs) -> String {
    let mut instruction: String = String::new();
    
    let chosen_type = var_type::random_basic_type(random);

    let instr_string = generate_type_instruction(random, bloc_variables, list_functions, list_structs, &chosen_type);

    match bloc_variables.get_random_variable(random, [chosen_type.clone()].to_vec(), true) {
        Some(assigned_var) => instruction = format!("{}{} = ", instruction, assigned_var.name_and_way(random, &chosen_type)),
        None => {
            let assigned_var = Variable::new(bloc_variables.next_variable_name(), random.gen_bool(), &chosen_type);
            instruction = format!("{}{} = ", instruction, assigned_var.initialize());
            bloc_variables.add_variable(assigned_var);
        },
    }

    format!("{}{};\n", instruction, instr_string)

}