use std::cmp::min;

use crate::{constants::{MAX_COMPOSITE_DEPTH, MAX_COMPOSITE_SIZE}, random::Random};

use super::{struct_type::StructType, var_type::random_type_with_depth};

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

    pub fn add_struct(&mut self, random: &mut Random) -> String {

        let size = random.gen_range(1, MAX_COMPOSITE_SIZE);
        let mut key_types = Vec::with_capacity(size);
        for i in 0..size {
            key_types.push((random_type_with_depth(random, self, min(1, MAX_COMPOSITE_DEPTH)).clone(), format!("elem{}", i+1)));
        }
        let new_strct = StructType::new(key_types, format!("strct{}", self.next_id()));
        let ret = new_strct.generate_struct_code();
        self.structs.push(new_strct);

        ret
    }

    pub fn generate_structs_code(&self) -> String {
        let mut ret = String::new();
        for i in &self.structs {
            ret = format!("{}{}", ret, i.generate_struct_code());
        }
        ret
    }

    fn next_id(&self) -> usize {
        self.structs.len()+1
    }
}