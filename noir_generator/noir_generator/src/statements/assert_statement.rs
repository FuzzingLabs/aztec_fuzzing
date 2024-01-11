use crate::variables::bloc_variables::BlocVariables;
use crate::variables::var_type;
use crate::random;

pub fn generate_assert_instruction(bloc_variables: &mut BlocVariables) -> String {
    let mut instruction: String = String::new();
    let mut variables_used = bloc_variables.get_variables_by_types(var_type::basic_types());

    instruction = format!("{}assert({}", instruction, variables_used[0].name());

    let mut bracket_count = 0;
    for var in variables_used.iter().skip(1) {
        let mut negation = "";
        let operator = random::choose_random_item_from_vec(&var_type::supported_comparator_operator_by_type(var.var_type().clone()));
        if random::gen_bool() { negation = "!" };

        match random::gen_range(0, 3) {
            0 => instruction = format!("{} {} {}{}", instruction, operator, negation, var.name()),
            1 => {
                bracket_count  += 1;
                instruction = format!("{} {} ({}{}", instruction, operator, negation, var.name());
            },
            2 => {
                if bracket_count != 0 {
                    bracket_count  -= 1;
                    instruction = format!("{} {} {}{})", instruction, operator, negation, var.name());
                } else {
                    bracket_count  += 1;
                    instruction = format!("{} {} ({}{}", instruction, operator, negation, var.name());
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