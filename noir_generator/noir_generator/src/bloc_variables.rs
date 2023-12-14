use crate::variable::Variable;
use crate::random;

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

    pub fn new_variable(&mut self, allowed_types: Vec<String>, visible: Option<bool>, mutable: Option<bool>) -> Option<&Variable>{
        let new_var = Variable::new(
            format!("var{}", self.next_id()),
            visible,
            mutable,
            allowed_types,
            None,
        );

        self.add_variable(new_var);

        self.variables.last()
    }

    pub fn get_random_variable(&mut self, allowed_types: Vec<String>, visible: Option<bool>, mutable: Option<bool>) -> Option<&Variable> {
        // Filtrer les variables en fonction des types autorisés
        let filtered_variables: Vec<&Variable> = self.variables
            .iter()
            .filter(|v| {
                let type_condition = allowed_types.contains(&v.type_());
        
                let visible_condition = visible.map_or(true, |value| v.is_public() == value);
                let mutable_condition = mutable.map_or(true, |value| v.is_mutable() == value);
        
                type_condition && visible_condition && mutable_condition
            })
            .collect();
    
        // Sélectionner de manière aléatoire une variable dans la liste filtrée
        random::choose_random_item_from_vec(&filtered_variables)
    }
    

    pub fn get_variable_by_name(&self, name: String) -> Option<Variable> {
        self.variables.iter().find(|var| *var.name() == name).cloned()
    }

    pub fn is_empty(&self) -> bool {
        return self.variables.len() == 0;
    }

    fn next_id(&self) -> usize {
        self.variables.len()+1
    }
}

