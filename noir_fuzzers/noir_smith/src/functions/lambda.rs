use crate::statements::random_statement;
use crate::tools::constants::CONFIG;
use crate::tools::random::Random;
use crate::variables::bloc_data::BlocData;
use crate::variables::list_structs::ListStructs;
use crate::variables::var_type::VarType;

use super::function::Function;
use super::list_functions::ListFunctions;

/// Represent a lambda function
#[derive(Clone)]
pub struct Lambda {
    func: Function,
}

impl Lambda {
    pub fn new(name: String, arguments: BlocData, ret_type: Option<VarType>) -> Self {
        Self {
            func: Function::new(name, true, arguments, ret_type),
        }
    }

    pub fn name(&self) -> &String {
        &self.func.name()
    }

    pub fn ret_type(&self) -> &Option<VarType> {
        &self.func.ret_type()
    }

    /// Return a string that represents the code needed for this lambda function
    pub fn initialize(
        &self,
        random: &mut Random,
        list_global: &BlocData,
        list_functions: &ListFunctions,
        list_structs: &ListStructs,
    ) -> String {
        let vars = self.func.arguments().variables();
        let mut init = format!("let {} = |", self.name());

        if vars.len() != 0 {
            for i in 0..vars.len() - 1 {
                init = format!("{}{}: {}, ", init, vars[i].name(), vars[i].var_type());
            }
            init = format!(
                "{}{}: {}",
                init,
                vars[vars.len() - 1].name(),
                vars[vars.len() - 1].var_type()
            );
        }

        init = format!("{}| {{\n", init);

        init = format!(
            "{}{}",
            init,
            self.generate_code(random, list_global, list_functions, list_structs)
        );

        format!("{}}};\n", init)
    }

    /// Return a string that represents the code needed to initialize this lambda function as argument
    pub fn initialize_as_argument(
        &self,
        random: &mut Random,
        list_global: &BlocData,
        list_functions: &ListFunctions,
        list_structs: &ListStructs,
    ) -> String {
        let vars = self.func.arguments().variables();
        let mut init = format!("|");

        if vars.len() != 0 {
            for i in 0..vars.len() - 1 {
                init = format!("{}{}: {}, ", init, vars[i].name(), vars[i].var_type());
            }
            init = format!(
                "{}{}: {}",
                init,
                vars[vars.len() - 1].name(),
                vars[vars.len() - 1].var_type()
            );
        }

        init = format!("{}| {{\n", init);

        init = format!(
            "{}{}",
            init,
            self.generate_code(random, list_global, list_functions, list_structs)
        );

        format!("{}}}", init)
    }

    /// Return a string that represents the code needed to use this lambda function as argument
    pub fn put_as_argument(&self) -> String {
        let vars = self.func.arguments().variables();
        let mut init = format!("{}: fn(", self.name());

        if vars.len() != 0 {
            for i in 0..vars.len() - 1 {
                init = format!("{}{}, ", init, vars[i].var_type());
            }
            init = format!("{}{}", init, vars[vars.len() - 1].var_type());
        }
        match self.ret_type() {
            Some(t) => format!("{}) -> {}", init, t),
            None => format!("{})", init),
        }
    }

    /// Replace generate_code of function.rs to match the core structure of a lambda function
    pub fn generate_code(
        &self,
        random: &mut Random,
        list_global: &BlocData,
        list_functions: &ListFunctions,
        list_structs: &ListStructs,
    ) -> String {
        let mut code = String::new();

        let mut bloc_variables = self.func.arguments().clone();
        for var in list_global.variables() {
            bloc_variables.add_variable(var);
        }

        let mut nb_instructions_left: usize = random.gen_range(0, CONFIG.max_instruction_by_lambda);
        while nb_instructions_left != 0 {
            nb_instructions_left -= 1;
            match random.gen_range(0, 7) {
                0 | 1 | 2 | 3 | 4 => {
                    code = format!(
                        "{}{}",
                        code,
                        random_statement::generate_random_statement(
                            random,
                            &mut bloc_variables,
                            list_global,
                            list_functions,
                            list_structs
                        )
                    )
                }
                5 => {
                    let nb_instruction_gived = random.gen_range(0, nb_instructions_left);
                    nb_instructions_left -= nb_instruction_gived;
                    code = format!(
                        "{}{}",
                        code,
                        Function::generate_if(
                            random,
                            list_global,
                            list_functions,
                            list_structs,
                            &bloc_variables,
                            nb_instruction_gived
                        )
                    );
                }
                6 => {
                    let nb_instruction_gived = random.gen_range(0, nb_instructions_left);
                    nb_instructions_left -= nb_instruction_gived;
                    code = format!(
                        "{}{}",
                        code,
                        Function::generate_for(
                            random,
                            list_global,
                            list_functions,
                            list_structs,
                            &bloc_variables,
                            nb_instruction_gived
                        )
                    );
                }
                // should never happen
                _ => {
                    code = format!(
                        "{}{}",
                        code,
                        random_statement::generate_random_statement(
                            random,
                            &mut bloc_variables,
                            list_global,
                            list_functions,
                            list_structs
                        )
                    )
                }
            }
        }

        format!(
            "{}{}",
            code,
            self.func.ret(
                random,
                &bloc_variables,
                list_global,
                list_functions,
                list_structs
            )
        )
    }

    pub fn call(
        &self,
        random: &mut Random,
        bloc_variables: &BlocData,
        list_global: &BlocData,
        list_functions: &ListFunctions,
        list_structs: &ListStructs,
        depth: usize,
    ) -> String {
        self.func.call(
            random,
            bloc_variables,
            list_global,
            list_functions,
            list_structs,
            depth,
        )
    }
}
