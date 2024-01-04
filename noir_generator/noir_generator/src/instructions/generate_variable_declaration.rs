use crate::variables::bloc_variables::BlocVariables;
use crate::variables::var_type;

pub fn generate_variable_declaration(bloc_variables: &mut BlocVariables) -> String {
    let typ = var_type::random_type();
    let new_variable = bloc_variables.new_variable([typ].to_vec(), None);
    new_variable.initialise()
}