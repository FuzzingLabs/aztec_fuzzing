use crate::random;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    name: String,
    visible: bool,
    mutable: bool,
    type_: &'static str,
    value: String,
}

impl Variable {
    pub fn new(name: String, visible: Option<bool>, mutable: Option<bool>, allowed_types: Vec<&'static str>, value: Option<String>) -> Self {
        let visible = match visible {
            Some(v) => v,
            None => random::generate_random_bool(),
        };

        let mutable = match mutable {
            Some(v) => v,
            None => random::generate_random_bool(),
        };

        let type_ = random::select_random_str_from_vec(allowed_types);

        let value = match value {
            Some(v) => v,
            None => random::generate_random_value_for_type(type_.clone()),
        };
        
        Self {
            name,
            visible,
            mutable,
            type_,
            value,
        }
    }

    pub fn is_public(&self) -> bool {
        self.visible
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
        format!("let{}{} {} : {} = {}\n", if self.is_public() { " pub" } else { "" }, if self.is_mutable() { " mut" } else { "" }, self.name(), self.type_(), self.value())
    }
}