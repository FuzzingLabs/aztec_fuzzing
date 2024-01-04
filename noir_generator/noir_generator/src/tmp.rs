mod random;
mod generate_function;
mod instructions;
mod variables;

use std::collections::BTreeMap;
use iter_extended::vecmap;

use noirc_frontend::parse_program;
use noirc_frontend::parser;
use noirc_frontend::hir::def_map::ModuleData;
use noirc_errors::Location;
use arena::Arena;
use fm::FileId;
use fm::FileManager;
use noirc_frontend::hir::def_map::{CrateDefMap, LocalModuleId};
use noirc_frontend::hir::Context;
use noirc_frontend::hir::def_collector::dc_crate::DefCollector;


const NB_MAX_FUNCTION: u32 = 10;

fn compile_code() -> Option<Vec<parser::ParserError>> {

    let mut code_generated = String::new();

    for _ in 0..random::gen_range(0, NB_MAX_FUNCTION) {
        code_generated = format!("{}{}\n", code_generated, generate_function::generate_function(random::gen_name()));
    }
    code_generated = format!("{}{}", code_generated, generate_function::generate_function("main".to_string()));

    let root = std::path::Path::new("/");
    let fm = FileManager::new(root);
    let mut context = Context::new(fm);
    let root_file_id = FileId::dummy();
    let root_crate_id = context.crate_graph.add_crate_root(root_file_id);
    let (program, parser_errors) = parse_program(&code_generated);
    let mut errors = vecmap(parser_errors, |e| (e.into(), root_file_id));
    remove_experimental_warnings(&mut errors);

    if !has_parser_error(&errors) {

        let mut modules: Arena<ModuleData> = Arena::default();
        let location = Location::new(Default::default(), root_file_id);
        let root = modules.insert(ModuleData::new(None, location, false));

        let def_map = CrateDefMap {
            root: LocalModuleId(root),
            modules,
            krate: root_crate_id,
            extern_prelude: BTreeMap::new(),
        };

        // Now we want to populate the CrateDefMap using the DefCollector
        errors.extend(DefCollector::collect(
            def_map,
            &mut context,
            program.clone().into_sorted(),
            root_file_id,
            Vec::new(), // No macro processors
        ));

        let nr_file_path = "/home/afredefon/FuzzingLabs/aztec_fuzzing/noir_generator/noir_generator/testNoir/test/src/main.nr"; // Choisissez un chemin approprié
        std::fs::write(nr_file_path, code_generated).expect("Failed to write temp file");
        return Some(errors);
    }

    None
}

fn main() {

    random::initialize_rng(None);
    
    let mut errors: Option<Vec<parser::ParserError>> = None;
    let mut compteur = 0;

    while let None = errors {
        errors = compile_code();

        compteur += 1;
        println!("nb loop: {}", compteur);
    }

    for error in &errors {
        println!("{:?}", error);
    }

}
