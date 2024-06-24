use crate::constants::CONFIG;
use crate::statements::random_statement;
use crate::variables::bloc_data::BlocData;
use crate::variables::list_structs::ListStructs;
use crate::variables::struct_type::StructType;
use crate::variables::var_type::VarType;
use crate::random::Random;
use crate::variables::variable::Variable;

use super::function::Function;
use super::list_functions::ListFunctions;

// Represent a method of a structure with a function and a boolean to indicate whether the method uses 'self' as an argument
// The name of the structure implementing this method is used to make calls to this method
#[derive(Clone)]
pub(crate) struct Method {
    func: Function,
    struct_source_name: String,
    self_arg: bool,
}

impl Method {
    pub fn new(name: String, arguments: BlocData, ret_type: Option<VarType>, struct_source_name: String, self_arg: bool) -> Self {
        Self {
            func: Function::new(name, true, arguments, ret_type),
            struct_source_name,
            self_arg,
        }
    }

    pub fn name(&self) -> &String {
        &self.func.name()
    }

    pub fn ret_type(&self) -> &Option<VarType> {
        &self.func.ret_type()
    }

    // Return a string that represents the code needed for this method
    pub fn initialize(&self, random: &mut Random, list_global: &BlocData, list_functions: &ListFunctions, list_structs: &ListStructs, struct_source: &StructType) -> String {
        let vars = self.func.arguments().variables();
        let mut init = format!("{}fn {}(", if self.func.is_public() { "pub " } else { "" }, self.name());

        init = format!("{}{}", init, if self.self_arg { if vars.len() == 0 {"self"} else {"self, "} } else { "" });

        if vars.len() != 0 {
            for i in 0..vars.len()-1{
                init = format!("{}{}: {}, ", init, vars[i].name(), vars[i].var_type());
            }
            init = format!("{}{}: {}", init, vars[vars.len()-1].name(), vars[vars.len()-1].var_type());
        }

        init = match &self.ret_type() {
            Some(v) => format!("{}) -> {} {{\n", init, v),
            None => format!("{}) {{\n", init),
        };

        init = format!("{}{}", init, self.generate_code(random, list_global, list_functions, list_structs, struct_source));
        
        format!("{}}}\n", init)
    }

    // Replace generate_code of function.rs to include the presence of 'self' as an argument
    pub fn generate_code(&self, random: &mut Random, list_global: &BlocData, list_functions: &ListFunctions, list_structs: &ListStructs, struct_source: &StructType) -> String {
        let mut code = String::new();
        let mut bloc_variables = self.func.arguments().clone();
        for var in list_global.variables() {
            bloc_variables.add_variable(var);
        }

        if self.self_arg {
            let var = Variable::new("self".to_string(), false, &VarType::strct(struct_source.clone()));
            bloc_variables.add_variable(var);
        }

        let mut nb_instructions_left: usize = random.gen_range(0, CONFIG.max_instruction_by_method);
        while nb_instructions_left != 0 {
            nb_instructions_left  -= 1;
            match random.gen_range(0, 7) {
                0 | 1 | 2 | 3 | 4 => code = format!("{}{}", code, random_statement::generate_random_statement(random, &mut bloc_variables, list_global, list_functions, list_structs)),
                5 => {
                    let nb_instruction_gived = random.gen_range(0, nb_instructions_left);
                    nb_instructions_left -= nb_instruction_gived;
                    code = format!("{}{}", code, Function::generate_if(random, list_global, list_functions, list_structs, &bloc_variables, nb_instruction_gived));
                },
                6 => {
                    let nb_instruction_gived = random.gen_range(0, nb_instructions_left);
                    nb_instructions_left -= nb_instruction_gived;
                    code = format!("{}{}", code, Function::generate_for(random, list_global, list_functions, list_structs, &bloc_variables, nb_instruction_gived));
                },
                // should never happen
                _ => code = format!("{}{}", code, random_statement::generate_random_statement(random, &mut bloc_variables, list_global, list_functions, list_structs)),
            }  
        }

        format!("{}{}", code, self.func.ret(random, &bloc_variables, list_global, list_functions, list_structs))
    }

    // Replace call of function.rs  to include the presence of 'self' as an argument
    pub fn call(&self, random: &mut Random, bloc_variables: &BlocData, list_global: &BlocData, list_functions: &ListFunctions, list_structs: &ListStructs, depth: usize, self_var: Option<String>) -> String {
        if self.self_arg {
            return format!("{}.{}", self_var.expect("No self_var in a method call"), self.func.call(random, list_global, bloc_variables, list_functions, list_structs, depth));
        }

        format!("{}::{}", self.struct_source_name, self.func.call(random, list_global, bloc_variables, list_functions, list_structs, depth))
    }
    
}