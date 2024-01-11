use crate::variables::variable::Variable;
use crate::variables::var_type::VarType;
use crate::random;

const MAX_VARIABLES_USABLE: usize = 10;

pub struct BlocVariables{
    variables: Vec<Variable>
}

impl BlocVariables {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    fn add_variable(&mut self, variable: Variable) {
        self.variables.push(variable)
    }

    pub fn new_variable(&mut self, allowed_types: Vec<VarType>, mutable: Option<bool>) -> Variable{
        let new_var = Variable::new(
            format!("var{}", self.next_id()),
            mutable,
            allowed_types,
        );

        self.add_variable(new_var.clone());

        new_var
    }

    pub fn get_random_variable(&mut self, allowed_types: Vec<VarType>, mutable: Option<bool>) -> Option<&Variable> {
        let filtered_variables: Vec<&Variable> = self.variables
            .iter()
            .filter(|v| {
                let type_condition = allowed_types.contains(&v.var_type());
                let mutable_condition = mutable.map_or(true, |value| v.is_mutable() == value);
        
                type_condition && mutable_condition
            })
            .collect();

        if filtered_variables.len() == 0 {
            return None;
        }
    
        Some(random::choose_random_item_from_vec(&filtered_variables))
    }

    pub fn get_variables_by_types(&self, target_types: Vec<VarType>) -> Vec<Variable> {
        self.variables
            .iter()
            .filter(|v| target_types.contains(&v.var_type()))
            .cloned()
            .take(MAX_VARIABLES_USABLE)
            .collect()
    }

    fn next_id(&self) -> usize {
        self.variables.len()+1
    }
}