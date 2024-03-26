use crate::variables::var_type::VarType;
use crate::random::Random;

use super::var_type;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn initialize(&self) -> String{
        format!("let{} {}: {}", if self.is_mutable() { " mut" } else { "" }, self.name(), self.var_type())
    }

    pub fn initialize_as_global(&self) -> String{
        format!("global {}: {}", self.name(), self.var_type())
    }

    pub fn name_and_way(&self, random: &mut Random, aim_type: &VarType) -> String {
        if let Some(str) = var_type::way_to_type(random, &self.var_type(), &aim_type) {
            return format!("{}{}", self.name(), str);
        } else {
            panic!("No way to type in name_and_way");
        }
        
    }

    
}