use std::cmp::min;

use crate::{constants::CONFIG, functions::list_functions::ListFunctions, random::Random};

use super::{bloc_data::BlocData, struct_type::StructType, var_type::{random_type_with_depth, VarType}, variable::Variable};

pub struct ListStructs{
    structs: Vec<StructType>
}

impl ListStructs {
    pub fn new() -> Self {
        Self {
            structs: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.structs.is_empty()
    }

    pub fn get_random(&self, random: &mut Random) -> StructType {
        random.choose_random_item_from_vec(&self.structs)
    }

    pub fn add_struct(&mut self, random: &mut Random, list_global: &BlocData, list_functions: &ListFunctions) -> String {

        let size = random.gen_range(1, CONFIG.max_composite_size);
        let mut key_types = Vec::with_capacity(size);
        for i in 0..size {
            key_types.push((random_type_with_depth(random, self, min(1, CONFIG.max_composite_depth)).clone(), format!("elem{}", i+1)));
        }
        let mut new_strct = StructType::new(key_types, format!("strct{}", self.next_id()));
        let ret = new_strct.generate_struct_code(random, list_global, list_functions, &self);

        self.structs.push(new_strct);

        ret
    }

    pub fn get_struct_variables(&self, random: &mut Random, bloc_data: &BlocData) -> Vec<Variable> {
        let mut ret_vec = Vec::new();
    
        for s in &self.structs {
            if let Some(v) = bloc_data.get_random_variable(random, vec![&VarType::strct(s.clone())], false) {
                ret_vec.push(v.clone());
            }
        }
    
        ret_vec
    }
    

    fn next_id(&self) -> usize {
        self.structs.len()+1
    }
}