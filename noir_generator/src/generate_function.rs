use crate::constants::NB_MAX_INSTRUCTION_BY_FUNCTION;
use crate::instructions::comparison_instruction::generate_comparison_instruction;
use crate::statements::random_statement;
use crate::variables::bloc_variables::{BlocVariables, self};
use crate::random;
use crate::variables::operand::Operand;
use crate::variables::variable::Variable;
use crate::variables::{value, variable};
use crate::variables::var_type::{self, VarType};

fn generate_else(fun_bloc_variables: &BlocVariables, nb_instructions_left: &mut i32) -> String {
    let mut bloc_variables = fun_bloc_variables.clone();

    let mut function: String = format!("}} else {{\n");

    while *nb_instructions_left > 0 {
        match random::gen_range(0, 7){
            0 | 1 | 2 | 3 | 4 => function = format!("{}{}", function, random_statement::generate_random_statement(&mut bloc_variables)),
            5 => function = format!("{}{}", function, generate_if(&bloc_variables, nb_instructions_left)),
            6 => function = format!("{}{}", function, generate_for(&bloc_variables, nb_instructions_left)),
            // should never happen
            _ => function = format!("{}{}", function, random_statement::generate_random_statement(&mut bloc_variables)),
        }
        *nb_instructions_left  -= 1;
    }

    function
}

fn generate_if(fun_bloc_variables: &BlocVariables, fun_nb_instructions_left: &mut i32) -> String {
    let mut bloc_variables = fun_bloc_variables.clone();
    let mut nb_instructions_left = random::gen_range(0, *fun_nb_instructions_left);
    *fun_nb_instructions_left -= nb_instructions_left;

    let mut function: String = format!("if {} {{\n", generate_comparison_instruction(&mut bloc_variables));

    while nb_instructions_left > 0 {
        nb_instructions_left  -= 1;
        match random::gen_range(0, 8){
            0 | 1 | 2 | 3 | 4 => function = format!("{}{}", function, random_statement::generate_random_statement(&mut bloc_variables)),
            5 => function = format!("{}{}", function, generate_if(&bloc_variables, &mut nb_instructions_left)),
            6 => function = format!("{}{}", function, generate_for(&bloc_variables, &mut nb_instructions_left)),
            7 => function = format!("{}{}", function, generate_else(fun_bloc_variables, &mut nb_instructions_left)),
            // should never happen
            _ => function = format!("{}{}", function, random_statement::generate_random_statement(&mut bloc_variables)),
        }
    }

    format!("{}}}\n", function)
}

fn generate_for(fun_bloc_variables: &BlocVariables, fun_nb_instructions_left: &mut i32) -> String {
    let mut bloc_variables = fun_bloc_variables.clone();
    let mut nb_instructions_left = random::gen_range(0, *fun_nb_instructions_left);
    *fun_nb_instructions_left -= nb_instructions_left;
    let chosen_type = &VarType::uint(64);

    let start_for = if random::gen_bool() {
        let var = bloc_variables.get_random_variable([chosen_type.clone()].to_vec(), None);
        match var {
            Some(v) => Operand::Variable(v.clone()),
            None => Operand::Value(value::random_value(chosen_type), chosen_type.clone()),
        }
    } else {
        Operand::Value(value::random_value(chosen_type), chosen_type.clone())
    };

    let end_for = if random::gen_bool() {
        let var = bloc_variables.get_random_variable([chosen_type.clone()].to_vec(), None);
        match var {
            Some(v) => Operand::Variable(v.clone()),
            None => Operand::Value(value::random_value(chosen_type), chosen_type.clone()),
        }
    } else {
        Operand::Value(value::random_value(chosen_type), chosen_type.clone())
    };

    let var = bloc_variables.new_variable(vec!(&VarType::uint(64)), Some(false));

    let mut function: String = format!("for {} in {}..{} {{\n", var.name(), 
    match start_for{
        Operand::Variable(v) => v.name_and_way(chosen_type),
        Operand::Operation(v) => format!("{}", v),
        Operand::Value(v, _) => format!("{}", v),
    }, 
    match end_for{
        Operand::Variable(v) => v.name_and_way(chosen_type),
        Operand::Operation(v) => format!("{}", v),
        Operand::Value(v, _) => format!("{}", v),
    });

    while nb_instructions_left > 0 {
        nb_instructions_left  -= 1;
        match random::gen_range(0, 6){
            0 | 1 | 2 | 3 | 4 => function = format!("{}{}", function, random_statement::generate_random_statement(&mut bloc_variables)),
            5 => function = format!("{}{}", function, generate_if(&bloc_variables, &mut nb_instructions_left)),
            6 => function = format!("{}{}", function, generate_for(&bloc_variables, &mut nb_instructions_left)),
            // should never happen
            _ => function = format!("{}{}", function, random_statement::generate_random_statement(&mut bloc_variables)),
        }
    }

    format!("{}}}\n", function)
}

pub fn generate_function(function_name: String) -> String{
    let mut function: String = format!("fn {}() {{\n", function_name);
    let mut bloc_variables = BlocVariables::new();
    let mut nb_instructions_left: i32 = random::gen_range(0, NB_MAX_INSTRUCTION_BY_FUNCTION.try_into().unwrap());

    while nb_instructions_left != 0 {
        nb_instructions_left  -= 1;
        match random::gen_range(0, 7){
            0 | 1 | 2 | 3 | 4 => function = format!("{}{}", function, random_statement::generate_random_statement(&mut bloc_variables)),
            5 => function = format!("{}{}", function, generate_if(&bloc_variables, &mut nb_instructions_left)),
            6 => function = format!("{}{}", function, generate_for(&bloc_variables, &mut nb_instructions_left)),
            // should never happen
            _ => function = format!("{}{}", function, random_statement::generate_random_statement(&mut bloc_variables)),
        }  
    }
    
    format!("{}}}\n", function)
}