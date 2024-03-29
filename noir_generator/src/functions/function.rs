use crate::{constants::{MAX_INSTRUCTION_BY_FUNCTION, MAX_LOOP_IN_FOR}, instructions::{comparison_instruction::generate_comparison_instruction, type_instruction::generate_type_instruction}, random::Random, statements::random_statement, variables::{bloc_data::BlocData, list_structs::{self, ListStructs}, var_type::VarType, variable::Variable}};

use super::list_functions::ListFunctions;



#[derive(Clone)]
pub(crate) struct Function {
    name: String,
    public: bool,
    arguments: BlocData,
    ret_type: Option<VarType>,
}

impl Function {
    pub fn new(name: String, public: bool, arguments: BlocData, ret_type: Option<VarType>) -> Self {

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

    pub fn arguments(&self) -> &BlocData {
        &self.arguments
    }

    pub fn initialize(&self) -> String {
        let vars = self.arguments.variables();
        let lambdas = self.arguments.lambdas();

        let mut init = format!("{}fn {}(", if self.public { "pub " } else { "" }, self.name());

        if vars.len() != 0 {
            for i in 0..vars.len()-1 {
                init = format!("{}{}: {}, ", init, vars[i].name(), vars[i].var_type());
            }

            if lambdas.len() == 0 {
                init = format!("{}{}: {}", init, vars[vars.len()-1].name(), vars[vars.len()-1].var_type());
            } else {
                init = format!("{}{}: {}, ", init, vars[vars.len()-1].name(), vars[vars.len()-1].var_type());
            }
        }

        if lambdas.len() != 0 {
            for i in 0..lambdas.len()-1 {
                init = format!("{}{}, ", init, lambdas[i].put_as_argument());
            }
            init = format!("{}{}", init, lambdas[lambdas.len()-1].put_as_argument());
        }

        match &self.ret_type {
            Some(v) => return format!("{}) -> {} {{\n", init, v),
            None => return format!("{}) {{\n", init),
        }

    }

    pub fn call(&self, random: &mut Random, bloc_variables: &BlocData, list_functions: &ListFunctions, list_structs: &ListStructs) -> String {
        let vars = self.arguments.variables();
        let lambdas = self.arguments.lambdas();

        let mut call = format!("{}(", self.name());

        if vars.len() != 0 {
            for i in 0..vars.len()-1 {
                call = format!("{}{}, ", call,  generate_type_instruction(random, bloc_variables, list_functions, list_structs, vars[i].var_type()));
            }

            if lambdas.len() == 0 {
                call = format!("{}{}", call,  generate_type_instruction(random, bloc_variables, list_functions, list_structs, vars[vars.len() -1].var_type()));
            } else {
                call = format!("{}{}, ", call,  generate_type_instruction(random, bloc_variables, list_functions, list_structs, vars[vars.len() -1].var_type()));
            }
        }

        if lambdas.len() != 0 {
            for i in 0..lambdas.len()-1 {
                call = format!("{}{}, ", call, lambdas[i].initialize_as_argument(random, list_functions, list_structs))
            }
            call = format!("{}{}", call, lambdas[lambdas.len()-1].initialize_as_argument(random, list_functions, list_structs))
        }

        format!("{})", call)
    }

    pub fn ret(&self, random: &mut Random, bloc_variables: &BlocData, list_functions: &ListFunctions, list_structs: &ListStructs) -> String {
        match self.ret_type() {
            Some(v) => {
                match v {
                    VarType::tup(_) => format!("{}\n", generate_type_instruction(random, bloc_variables, list_functions, list_structs, v)),
                    _ => {
                        let mut ret = generate_type_instruction(random, bloc_variables, list_functions, list_structs, v);
                        if ret.starts_with("(") && ret.ends_with(")") {
                            ret.remove(0);
                            ret.pop();
                        }
                        format!("{}\n", ret)
                    }
                }
                
            }
            None => "".to_string(),
        }
    }

    pub fn generate_function_code(&self, random: &mut Random, list_global: &BlocData, list_functions: &ListFunctions, list_structs: &ListStructs) -> String {
        let mut bloc_variables = self.arguments.clone();
        for var in list_global.variables() {
            bloc_variables.add_variable(var);
        }
        let mut function_string: String = self.initialize();

        let mut nb_instructions_left: usize = random.gen_range(0, MAX_INSTRUCTION_BY_FUNCTION);
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

        function_string = format!("{}{}", function_string, self.ret(random, &bloc_variables, list_functions, list_structs));
        
        format!("{}}}\n\n", function_string)
    }

    pub fn generate_else(random: &mut Random, list_functions: &ListFunctions, list_structs: &ListStructs, fun_bloc_variables: &BlocData, nb_instructions_left: &mut usize) -> String {
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
    
    pub fn generate_if(random: &mut Random, list_functions: &ListFunctions, list_structs: &ListStructs, fun_bloc_variables: &BlocData, fun_nb_instructions_left: &mut usize) -> String {
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
    
    pub fn generate_for(random: &mut Random, list_functions: &ListFunctions, list_structs: &ListStructs, fun_bloc_variables: &BlocData, fun_nb_instructions_left: &mut usize) -> String {
        let mut bloc_variables = fun_bloc_variables.clone();
        let mut nb_instructions_left = random.gen_range(0, *fun_nb_instructions_left);
        *fun_nb_instructions_left -= nb_instructions_left;
        let chosen_type = &VarType::uint(32);
    
        
        let start_for = random.gen_random_int(32).try_into().unwrap();
        let end_for = random.gen_range(start_for, start_for+MAX_LOOP_IN_FOR);
    
        let var = Variable::new(bloc_variables.next_variable_name(), false, chosen_type);
        bloc_variables.add_variable(var.clone());
        let mut function_string: String = format!("for {} in {}..{} {{\n", var.name(), start_for, end_for);
    
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