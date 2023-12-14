use crate::bloc_variables::BlocVariables;
use crate::variable::Variable;
use crate::types;
use crate::random;

pub fn generate_random_instruction(mut bloc_variables: BlocVariables) -> String {
    let mut instruction: String = String::new();

    let mut variables: Vec<Option<Variable>> = Vec::new();

    for i in 0..10 {
        match bloc_variables.get_random_variable(types::types(), None, None) {
            Some(variable) => {
                variables.push(Some(variable.clone()));
            }
            None => {
                if let Some(new_variable) = bloc_variables.new_variable(types::types(), None, None) {
                    instruction = format!("{}{}", instruction, new_variable.initialise());
                    variables.push(Some(new_variable.clone()));
                } else {
                    // Gérer le cas où aucune nouvelle variable n'a été créée
                    panic!("Impossible de créer une nouvelle variable.");
                }
            }
        }
    }

    // Utilisez la première variable différemment
    if let Some(first_variable) = variables.get(0) {
        if let Some(variable) = first_variable {
            instruction = format!("{}{}", instruction, variable.name());
        } else {
            // Gérer le cas où la première variable n'est pas initialisée
            panic!("Impossible de continuer avec une première variable non initialisée.");
        }
    }

    // Utilisez toutes les variables dans la génération d'instruction
    for var_option in variables.iter().skip(1) {
        if let Some(variable) = var_option {
            instruction = format!(
                "{} {} {}",
                instruction,
                random::select_random_string_from_vec(&types::operators()),
                variable.name()
            );
        } else {
            // Gérer le cas où une variable n'est pas initialisée
            panic!("Impossible de continuer avec une variable non initialisée.");
        }
    }

    instruction + "\n"
}
