use crate::variables::var_type::VarType;
use crate::random;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Variable {
    name: String,
    var_type: VarType,
    mutable: bool,
}

impl Variable {
    pub fn new(name: String, mutable: Option<bool>, allowed_types: Vec<VarType>) -> Self {

        let mutable = match mutable {
            Some(v) => v,
            None => random::gen_bool(),
        };

        let var_type = random::choose_random_item_from_vec(&allowed_types);
        
        Self {
            name,
            mutable,
            var_type,
        }
    }

    pub fn var_type(&self) -> VarType {
        self.var_type.clone()
    }
    
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn initialise(&self) -> String{
        format!("let{} {}: {}", if self.is_mutable() { " mut" } else { "" }, self.name(), self.var_type())
    }
}