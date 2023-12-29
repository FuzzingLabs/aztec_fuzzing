use crate::variables::bloc_variables::BlocVariables;
use crate::variables::types;

pub fn generate_variable_declaration(bloc_variables: &mut BlocVariables) -> String {
    let new_variable = bloc_variables.new_variable(types::basic_types(), None);
    new_variable.initialise()
}