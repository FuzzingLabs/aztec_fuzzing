use crate::variables::var_type::VarType;
use crate::random::Random;

use super::var_type;

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

    pub fn initialize(&self) -> String{
        format!("let{} {}: {}", if self.is_mutable() { " mut" } else { "" }, self.name(), self.var_type())
    }

    pub fn initialize_as_global(&self) -> String{
        format!("global {}: {}", self.name(), self.var_type())
    }

    pub fn name_and_way(&self, random: &mut Random, aim_type: &VarType) -> String {
        let mut is_a_ref = false;
        if let Some(str) = var_type::way_to_type(random, &self.var_type(), &aim_type, &mut is_a_ref) {
            return format!("{}{}{}", if is_a_ref { "*" } else { "" }, self.name(), str);
        } else {
            panic!("No way to type in name_and_way");
        }
        
    }

    
}