use crate::random;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    name: String,
    mutable: bool,
    type_: &'static str,
    value: String,
}

impl Variable {
    pub fn new(name: String, mutable: Option<bool>, allowed_types: Vec<&'static str>, value: Option<String>) -> Self {

        let mutable = match mutable {
            Some(v) => v,
            None => random::generate_random_bool(),
        };

        let type_ = random::select_random_str_from_vec(allowed_types);

        let value = match value {
            Some(v) => v,
            None => random::generate_random_value_for_basic_type(type_.clone()),
        };
        
        Self {
            name,
            mutable,
            type_,
            value,
        }
    }
    
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }
    
    pub fn type_(&self) -> &'static str {
        self.type_
    }
    
    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn initialise(&self) -> String{
        format!("let{} {}: {} = {};\n", if self.is_mutable() { " mut" } else { "" }, self.name(), self.type_(), self.value())
    }
}