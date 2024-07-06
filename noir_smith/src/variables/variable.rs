use crate::variables::var_type::VarType;
use crate::tools::random::Random;

use super::var_type;

/// Represents a variable by its name, type, and mutability status
#[derive(Clone)]
pub(crate) struct Variable {
    name: String,
    var_type: VarType,
    mutable: bool,
}

impl Variable {
    pub fn new(name: String, mutable: bool, var_type: &VarType) -> Self { 
        Self {
            name,
            mutable,
            var_type: var_type.clone(),
        }
    }

    pub fn var_type(&self) -> &VarType {
        &self.var_type
    }
    
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_type(&mut self, type_to_set: &VarType) {
        self.var_type = type_to_set.clone();
    }

    /// Returns a string representing the statement that initializes this variable
    pub fn initialize(&self) -> String {
        format!("let{} {}: {}", if self.is_mutable() { " mut" } else { "" }, self.name(), self.var_type())
    }

    /// Returns a string representing the statement that initializes this variable as a global variable
    pub fn initialize_as_global(&self) -> String {
        format!("global {}: {}", self.name(), self.var_type())
    }

    /// Returns a string representing a statement of the same type as aim_type using this variable
    /// # Example
    /// If var has type [u8; 1] and aim_type is u8, the result might be var[0]
    pub fn name_and_way(&self, random: &mut Random, aim_type: &VarType) -> String {
        let mut is_a_ref = false;
        if let Some(str) = var_type::way_to_type(random, &self.var_type(), &aim_type, &mut is_a_ref) {
            return format!("{}{}{}", if is_a_ref { "*" } else { "" }, self.name(), str);
        } else {
            panic!("No way to convert type in name_and_way");
        }
    }

    
}