use crate::variables::bloc_variables::BlocVariables;
use crate::variables::variable::Variable;
use crate::variables::types;
use crate::random;
use crate::instructions::generate_instruction::fill_bloc_variables;

pub fn generate_assert_instruction(bloc_variables: &mut BlocVariables) -> String {
    let mut instruction: String = String::new();
    let mut variables_used: Vec<Variable> = Vec::new();

    fill_bloc_variables(bloc_variables, &mut variables_used, types::basic_types());
    instruction = format!("{}{}", instruction, fill_bloc_variables(bloc_variables, &mut variables_used, types::basic_types()));

    instruction = format!("{}assert({}", instruction, variables_used[0].name());

    let mut bracket_count = 0;
    for var in variables_used.iter().skip(1) {
        let mut negation = "";
        let operand = random::select_random_str_from_vec(types::supported_operations_for_assertion(var.type_()));
        if random::generate_random_bool()  { negation = "!" };

        match random::generate_random_number(0, 3) {
            0 => instruction = format!("{} {} {}{}", instruction, operand, negation, var.name()),
            1 => {
                bracket_count  += 1;
                instruction = format!("{} {} ({}{}", instruction, operand, negation, var.name());
            },
            2 => {
                if bracket_count != 0 {
                    bracket_count  -= 1;
                    instruction = format!("{} {} {}{})", instruction, operand, negation, var.name());
                } else {
                    bracket_count  += 1;
                    instruction = format!("{} {} ({}{}", instruction, operand, negation, var.name());
                } 
            }
            _ => {}
        }
    }

    for _ in 0..bracket_count {
        instruction = format!("{})", instruction);
    }

    instruction + ");\n"
}