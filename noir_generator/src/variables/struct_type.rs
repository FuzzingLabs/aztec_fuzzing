use super::var_type::VarType;

#[derive(Debug, Clone, PartialEq)]
pub struct StructType{
    key_types: Vec<(VarType, String)>,
    name: String,
}

impl StructType {
    pub fn new(key_types: Vec<(VarType, String)>, name: String) -> Self {
        Self {
            key_types,
            name,
        }
    }

    pub fn key_types(&self) -> &Vec<(VarType, String)> {
        &self.key_types
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn generate_struct_code(&self) -> String {
        let mut struct_string: String = String::new();
        struct_string = format!("{}struct {} {{\n", struct_string, self.name);
        for (_, key_type) in self.key_types.iter().enumerate() {
            struct_string = format!("{}{}: {},\n", struct_string, key_type.1, key_type.0);
        }
        format!("{}}}\n\n", struct_string)
    }
}