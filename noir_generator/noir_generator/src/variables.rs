use std::sync::{Mutex, MutexGuard};

use rand::Rng;
use regex::Regex;

use crate::{variable::Variable, values};

pub struct Variables;

// static POSSIBLE_VARIABLE_VISIBILITIES: [&str; 2] = ["pub", ""]; // Seems like Noir doesn't support pub
// static POSSIBLE_VARIABLE_GROUPS: [&str; 2] = ["let", "const"]; not yet implemented on Noir // Seems like Noir doesn't support const

impl Variables {
    pub fn new_random_basic_variable() -> String{
        let name = format!("var{}", Self::next_id());
        let visible = rand::thread_rng().gen::<bool>();
        let mutable = rand::thread_rng().gen::<bool>();
        let mut type_ = values::generate_variable_type();
        let value = values::generate_random_value_from_type(type_.clone());

        if type_ == "str" {
            type_ = format!("str<{}>", value.len()-2); // -2 because of the ""
        }

        Self::add_variable(
            name.clone(),
            visible,
            mutable,
            type_.clone(),
            value.clone(),
        );
        return format!("let{} {} : {} = {}", if mutable { " mut" } else { "" }, name, type_, value);
    }

    pub fn new_random_basic_variables(amount: usize) -> String{
        let mut result: String = String::new();
        for _ in 0..amount {
            result += &Self::new_random_basic_variable();
        }
        return result;
    }

    pub fn new_random_basic_variable_with_type(type_: String) -> String{
        let name = format!("var{}", Self::next_id());
        let visible = rand::thread_rng().gen::<bool>();
        let mutable = rand::thread_rng().gen::<bool>();
        let value: String; 
        if Self::type_is_str(type_.clone()) {
            let lenght = Self::get_lenght_from_str(type_.clone());
            value = values::generate_random_str_with_length(lenght);
        } else {
            value = values::generate_random_value_from_type(type_.clone());
        }

        Self::add_variable(
            name.clone(),
            visible,
            mutable,
            type_.clone(),
            value.clone(),
        );
        return format!("let{} {} : {} = {}", if mutable { " mut" } else { "" }, name, type_, value);
    }

    pub fn new_random_basic_variable_except_type(type_: Vec<String>) -> String {
        let name = format!("var{}", Self::next_id());
        let visible = rand::thread_rng().gen::<bool>();
        let mutable = rand::thread_rng().gen::<bool>();
        let type_ = values::generate_variable_type_except_types(type_.clone());
        let value = values::generate_random_value_from_type(type_.clone());

        Self::add_variable(
            name.clone(),
            visible,
            mutable,
            type_.clone(),
            value.clone(),
        );
        return format!("let{} {} : {} = {}", if mutable { " mut" } else { "" }, name, type_, value);
    }

    pub fn get_variables() -> MutexGuard<'static, Vec<Variable>> {
        VARIABLES.lock().unwrap()
    }

    pub fn get_variable_by_name(name: String) -> Option<Variable> {
        let variables = Self::get_variables();
        for variable in variables.iter() {
            if variable.get_name() == name {
                return Some(variable.clone());
            }
        }
        return None;
    }

    pub fn get_random_basic_variable() -> Option<Variable> {
        if Self::is_empty() {
            return None;
        }
        let variables = Self::get_variables();
        let index = rand::thread_rng().gen_range(0..variables.len());
        return Some(variables[index].clone());
    }

    pub fn get_random_variable_with_type(type_: String) -> Option<Variable> {
        if Self::is_empty() {
            return None;
        }
        if values::verify_type_existence(type_.clone()) {
            if Self::verify_this_type_is_in_variables(type_.clone()) {
                let variables = Self::get_variables();
                let mut index = rand::thread_rng().gen_range(0..variables.len());
                while variables[index].get_type() != type_ {
                    index = rand::thread_rng().gen_range(0..variables.len());
                }
                return Some(variables[index].clone());
            } else {
                return None;
            }
        }
        return None;
    }

    pub fn get_random_variable_except_types(except_types: Vec<String>) -> Option<Variable> {
        if Self::is_empty() {
            return None;
        }
        if !Self::has_variables_except_types(except_types.clone()) {
            return None;
        }
        let variables = Self::get_variables();
        let mut index = rand::thread_rng().gen_range(0..variables.len());
        while except_types.contains(&variables[index].get_type().to_string()) {
            index = rand::thread_rng().gen_range(0..variables.len());
        }
        return Some(variables[index].clone());
    }

    pub fn get_random_basic_mutable_variable() -> Option<Variable> {
        if Self::is_empty() {
            return None;
        }
        if !Self::verify_mutable_variable_in_variables(){
            return None;
        }
        let variables = Self::get_variables();
        let mut index = rand::thread_rng().gen_range(0..variables.len());
        while !variables[index].is_mutable() {
            index = rand::thread_rng().gen_range(0..variables.len());
        }
        return Some(variables[index].clone());
    }

    pub fn is_empty() -> bool {
        let variables = Self::get_variables();
        return variables.len() == 0;
    }

    pub fn has_variables_except_types(except_types: Vec<String>) -> bool {
        let variables = Self::get_variables();
        for variable in variables.iter() {
            if !except_types.contains(&variable.get_type().to_string()) {
                return true;
            }
        }
        return false;
    }

    pub fn has_variables_with_type(type_: String) -> bool {
        let variables = Self::get_variables();
        for variable in variables.iter() {
            if variable.get_type() == type_ {
                return true;
            }
        }
        return false;
    }

    pub fn get_name_from_string(input: String) -> Option<String> {
        let re = Regex::new(r"var(\d+)").unwrap();

        if let Some(captures) = re.captures(&input) {
            if let Some(matched) = captures.get(1) {
                let variable_name = "var".to_owned() + matched.as_str();
                return Some(variable_name.to_string());
            }
        }
        return None;
    }

    pub fn type_is_str(type_: String) -> bool {
        let re = Regex::new(r"str<(\d+)>").unwrap();

        if let Some(captures) = re.captures(&type_) {
            if let Some(matched) = captures.get(1) {
                return true;
            }
        }
        return false;
    }

    fn add_variable(name: String, visible: bool, mutable: bool, type_: String, value: String) {
        let mut variables_state = VARIABLES.lock().unwrap();
        variables_state.push(Variable::new(name, visible, mutable, type_, value));
    }

    fn next_id() -> usize {
        VARIABLES.lock().unwrap().len()+1
    }

    fn verify_this_type_is_in_variables(type_: String) -> bool {
        let variables = Self::get_variables();
        for variable in variables.iter() {
            if variable.get_type() == type_ {
                return true;
            }
        }
        return false;
    }

    fn verify_mutable_variable_in_variables() -> bool {
        let variables = Self::get_variables();
        for variable in variables.iter() {
            if variable.is_mutable() {
                return true;
            }
        }
        return false;
    }

    fn get_lenght_from_str(type_: String) -> usize {
        let re = Regex::new(r"str<(\d+)>").unwrap();

        if let Some(captures) = re.captures(&type_) {
            if let Some(matched) = captures.get(1) {
                return matched.as_str().parse::<usize>().unwrap();
            }
        }
        return 0;
    }
}

lazy_static! {
    pub static ref VARIABLES: Mutex<Vec<Variable>> = Mutex::new(Vec::new());
}