use crate::{constants::{self, NB_MAX_INSTRUCTION_BY_FUNCTION}, instructions::{comparison_instruction::generate_comparison_instruction, type_instruction::generate_type_instruction}, random::{self, Random}, statements::random_statement, variables::{bloc_variables::{self, BlocVariables}, list_structs::ListStructs, operand::Operand, value, var_type::{self, VarType}, variable::Variable}};

use super::list_functions::{self, ListFunctions};



#[derive(Clone)]
pub(crate) struct Function {
    name: String,
    public: bool,
    arguments: BlocVariables,
    ret_type: Option<VarType>,
}

impl Function {
    pub fn new(name: String, public: bool, arguments: BlocVariables, ret_type: Option<VarType>) -> Self {

        Self {
            name,
            public,
            arguments,
            ret_type,
        }
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn ret_type(&self) -> &Option<VarType> {
        &self.ret_type
    }

    pub fn initialize(&self) -> String {
        let vars = self.arguments.variables();
        let mut init = format!("{}fn {}(", if self.public { "pub " } else { "" }, self.name());

        for i in 0..vars.len()-1{
            init = format!("{}{}: {}, ", init, vars[i].name(), vars[i].var_type());
        }
        if vars.len() != 0 {
            init = format!("{}{}: {}", init, vars[vars.len()-1].name(), vars[vars.len()-1].var_type());
        }

        match &self.ret_type {
            Some(v) => return format!("{}) -> {} {{\n", init, v),
            None => return format!("{}) {{\n", init),
        }

    }

    pub fn call(&self, random: &mut Random) -> String {
        let vars = self.arguments.variables();
        let mut call = format!("{}(", self.name());

        for i in 0..vars.len()-1{
            call = format!("{}{}, ", call,  value::random_value(random, vars[i].var_type()));
        }
        if vars.len() != 0 {
            call = format!("{}{}", call,  value::random_value(random, vars[vars.len() -1].var_type()));
        }

        format!("{})", call)
    }

    pub fn ret(&self, random: &mut Random, bloc_variables: &BlocVariables, list_functions: &ListFunctions) -> String {
        match self.ret_type() {
            Some(v) => format!("{}\n", generate_type_instruction(random, &bloc_variables, list_functions, v)),
            None => format!(""),
        }
    }

    pub fn generate_function_code(&self, random: &mut Random, list_functions: &ListFunctions, list_structs: &ListStructs) -> String {
        let mut bloc_variables = self.arguments.clone();
        let mut function_string: String = self.initialize();

        let mut nb_instructions_left: usize = random.gen_range(0, NB_MAX_INSTRUCTION_BY_FUNCTION);
        while nb_instructions_left != 0 {
            nb_instructions_left  -= 1;
            match random.gen_range(0, 7){
                0 | 1 | 2 | 3 | 4 => function_string = format!("{}{}", function_string, random_statement::generate_random_statement(random, &mut bloc_variables, list_functions, list_structs)),
                5 => function_string = format!("{}{}", function_string, Self::generate_if(random, list_functions, list_structs, &bloc_variables, &mut nb_instructions_left)),
                6 => function_string = format!("{}{}", function_string, Self::generate_for(random, list_functions, list_structs, &bloc_variables, &mut nb_instructions_left)),
                // should never happen
                _ => function_string = format!("{}{}", function_string, random_statement::generate_random_statement(random, &mut bloc_variables, list_functions, list_structs)),
            }  
        }

        function_string = format!("{}{}", function_string, self.ret(random, &bloc_variables, list_functions));
        
        format!("{}}}\n\n", function_string)
    }

    fn generate_else(random: &mut Random, list_functions: &ListFunctions, list_structs: &ListStructs, fun_bloc_variables: &BlocVariables, nb_instructions_left: &mut usize) -> String {
        let mut bloc_variables = fun_bloc_variables.clone();
    
        let mut function_string: String = format!("}} else {{\n");
    
        while *nb_instructions_left > 0 {
            match random.gen_range(0, 7){
                0 | 1 | 2 | 3 | 4 => function_string = format!("{}{}", function_string, random_statement::generate_random_statement(random, &mut bloc_variables, list_functions, list_structs)),
                5 => function_string = format!("{}{}", function_string, Self::generate_if(random, list_functions, list_structs, &bloc_variables, nb_instructions_left)),
                6 => function_string = format!("{}{}", function_string, Self::generate_for(random, list_functions, list_structs, &bloc_variables, nb_instructions_left)),
                // should never happen
                _ => function_string = format!("{}{}", function_string, random_statement::generate_random_statement(random, &mut bloc_variables, list_functions, list_structs)),
            }
            *nb_instructions_left  -= 1;
        }
    
        function_string
    }
    
    fn generate_if(random: &mut Random, list_functions: &ListFunctions, list_structs: &ListStructs, fun_bloc_variables: &BlocVariables, fun_nb_instructions_left: &mut usize) -> String {
        let mut bloc_variables = fun_bloc_variables.clone();
        let mut nb_instructions_left = random.gen_range(0, *fun_nb_instructions_left);
        *fun_nb_instructions_left -= nb_instructions_left;
    
        let mut function_string: String = format!("if {}{{\n", generate_comparison_instruction(random, &mut bloc_variables));
    
        while nb_instructions_left > 0 {
            nb_instructions_left  -= 1;
            match random.gen_range(0, 8){
                0 | 1 | 2 | 3 | 4 => function_string = format!("{}{}", function_string, random_statement::generate_random_statement(random ,&mut bloc_variables, list_functions, list_structs)),
                5 => function_string = format!("{}{}", function_string, Self::generate_if(random ,list_functions, list_structs, &bloc_variables, &mut nb_instructions_left)),
                6 => function_string = format!("{}{}", function_string, Self::generate_for(random ,list_functions, list_structs, &bloc_variables, &mut nb_instructions_left)),
                7 => function_string = format!("{}{}", function_string, Self::generate_else(random ,list_functions, list_structs, fun_bloc_variables, &mut nb_instructions_left)),
                // should never happen
                _ => function_string = format!("{}{}", function_string, random_statement::generate_random_statement(random ,&mut bloc_variables, list_functions, list_structs)),
            }
        }
    
        format!("{}}}\n", function_string)
    }
    
    fn generate_for(random: &mut Random, list_functions: &ListFunctions, list_structs: &ListStructs, fun_bloc_variables: &BlocVariables, fun_nb_instructions_left: &mut usize) -> String {
        let mut bloc_variables = fun_bloc_variables.clone();
        let mut nb_instructions_left = random.gen_range(0, *fun_nb_instructions_left);
        *fun_nb_instructions_left -= nb_instructions_left;
        let chosen_type = &VarType::uint(8);
    
        let start_for = if random.gen_bool() {
            let random_bool = random.gen_bool();
            let var = bloc_variables.get_random_variable(random, [chosen_type.clone()].to_vec(), random_bool);
            match var {
                Some(v) => Operand::Variable(v.clone()),
                None => Operand::Value(value::random_value(random, chosen_type), chosen_type.clone()),
            }
        } else {
            Operand::Value(value::random_value(random, chosen_type), chosen_type.clone())
        };
    
        let end_for = if random.gen_bool() {
            let random_bool = random.gen_bool();
            let var = bloc_variables.get_random_variable(random, [chosen_type.clone()].to_vec(), random_bool);
            match var {
                Some(v) => Operand::Variable(v.clone()),
                None => Operand::Value(value::random_value(random, chosen_type), chosen_type.clone()),
            }
        } else {
            Operand::Value(value::random_value(random, chosen_type), chosen_type.clone())
        };
    
        let var = bloc_variables.new_variable(chosen_type, false);
    
        let mut function_string: String = format!("for {} in {}..{} {{\n", var.name(), 
        match start_for{
            Operand::Variable(v) => v.name_and_way(random, chosen_type),
            Operand::Operation(v) => format!("{}", v.to_string(random)),
            Operand::Value(v, _) => format!("{}", v),
        }, 
        match end_for{
            Operand::Variable(v) => v.name_and_way(random, chosen_type),
            Operand::Operation(v) => format!("{}", v.to_string(random)),
            Operand::Value(v, _) => format!("{}", v),
        });
    
        while nb_instructions_left > 0 {
            nb_instructions_left  -= 1;
            match random.gen_range(0, 6){
                0 | 1 | 2 | 3 | 4 => function_string = format!("{}{}", function_string, random_statement::generate_random_statement(random ,&mut bloc_variables, list_functions, list_structs)),
                5 => function_string = format!("{}{}", function_string, Self::generate_if(random ,list_functions, list_structs, &bloc_variables, &mut nb_instructions_left)),
                6 => function_string = format!("{}{}", function_string, Self::generate_for(random ,list_functions, list_structs, &bloc_variables, &mut nb_instructions_left)),
                // should never happen
                _ => function_string = format!("{}{}", function_string, random_statement::generate_random_statement(random ,&mut bloc_variables, list_functions, list_structs)),
            }
        }
    
        format!("{}}}\n", function_string)
    }

    
}