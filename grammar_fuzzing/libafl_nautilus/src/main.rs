use std::path::PathBuf;
#[cfg(windows)]
use std::ptr::write_volatile;

use fm::FileManager;
use libafl::{
    corpus::{InMemoryCorpus, OnDiskCorpus},
    events::SimpleEventManager,
    executors::{inprocess::InProcessExecutor, ExitKind},
    feedback_or,
    feedbacks::{CrashFeedback, MaxMapFeedback, NautilusChunksMetadata, NautilusFeedback},
    fuzzer::{Fuzzer, StdFuzzer},
    generators::{NautilusContext, NautilusGenerator},
    inputs::NautilusInput,
    monitors::SimpleMonitor,
    mutators::{
        NautilusRandomMutator, NautilusRecursionMutator, NautilusSpliceMutator, StdScheduledMutator,
    },
    observers::StdMapObserver,
    schedulers::QueueScheduler,
    stages::mutational::StdMutationalStage,
    state::{HasMetadata, StdState},
};
use libafl_bolts::{current_nanos, rands::StdRand, tuples::tuple_list};
use nargo::ops::compile_workspace;
use nargo_toml::{resolve_workspace_from_toml, PackageSelection};
use noirc_driver::{file_manager_with_stdlib, CompileOptions, NOIR_ARTIFACT_VERSION_STRING};
use noirc_frontend::{
    hir::{def_map::parse_file, ParsedFiles},
    parse_program,
};
use tools::ignored_error;

mod tools;

/// Coverage map with explicit assignments due to the lack of instrumentation
static mut SIGNALS: [u8; 16] = [0; 16];
static mut SIGNALS_PTR: *mut u8 = unsafe { SIGNALS.as_mut_ptr() };
/*
/// Assign a signal to the signals map
fn signals_set(idx: usize) {
    unsafe { str::ptr::write(SIGNALS_PTR.add(idx), 1) };
}
*/

fn parse_all(fm: &FileManager) -> ParsedFiles {
    fm.as_file_map()
        .all_file_ids()
        .map(|&file_id| (file_id, parse_file(fm, file_id)))
        .collect()
}

#[allow(clippy::similar_names)]
pub fn main() {
    let context = NautilusContext::from_file(300, "noir.json");
    let mut bytes = vec![];

    let noir_project_dir = std::env::current_dir().unwrap().join("noir_project");
    let nr_main_path = noir_project_dir.join("src/main.nr");

    let fm_stdlib = &file_manager_with_stdlib(std::path::Path::new(""));
    let parsed_files_stdlib = parse_all(&fm_stdlib);

    // The closure that we want to fuzz
    let mut harness = |input: &NautilusInput| {
        input.unparse(&context, &mut bytes);

        let mut fm = fm_stdlib.clone();
        let mut parsed_files = parsed_files_stdlib.clone();

        let parsed = parse_program(&String::from_utf8_lossy(bytes.as_slice()));
        let file_id =
            fm.add_file_with_source(&nr_main_path, String::from_utf8_lossy(&bytes).to_string());
        parsed_files.insert(file_id.expect("No file id"), parsed);

        let options = CompileOptions::default();

        let workspace = match resolve_workspace_from_toml(
            &noir_project_dir.join("Nargo.toml"),
            PackageSelection::DefaultOrAll,
            Some(NOIR_ARTIFACT_VERSION_STRING.to_string()),
        ) {
            Ok(w) => w,
            Err(_) => panic!("Can't resolve workspace from toml"),
        };

        match compile_workspace(&fm, &parsed_files, &workspace, &options) {
            Ok(_) => ExitKind::Ok,
            Err(errors) => {
                for error in errors.iter() {
                    if error.diagnostic.is_error() && !ignored_error(&error.diagnostic.message) {
                        panic!("{}", error.diagnostic.message);
                    }
                }
                ExitKind::Crash
            }
        }
    };

    // Create an observation channel using the signals map
    let observer = unsafe { StdMapObserver::from_mut_ptr("signals", SIGNALS_PTR, SIGNALS.len()) };

    // Feedback to rate the interestingness of an input
    let mut feedback = feedback_or!(
        MaxMapFeedback::new(&observer),
        NautilusFeedback::new(&context)
    );

    // A feedback to choose if an input is a solution or not
    let mut objective = CrashFeedback::new();

    // create a State from scratch
    let mut state = StdState::new(
        // RNG
        StdRand::with_seed(current_nanos()),
        // Corpus that will be evolved, we keep it in memory for performance
        InMemoryCorpus::new(),
        // Corpus in which we store solutions (crashes in this example),
        // on disk so the user can get them after stopping the fuzzer
        OnDiskCorpus::new(PathBuf::from("./crashes")).unwrap(),
        // States of the feedbacks.
        // The feedbacks can report the data that should persist in the State.
        &mut feedback,
        // Same for objective feedbacks
        &mut objective,
    )
    .unwrap();

    if state
        .metadata_map()
        .get::<NautilusChunksMetadata>()
        .is_none()
    {
        state.add_metadata(NautilusChunksMetadata::new("/tmp/".into()));
    }

    // The Monitor trait define how the fuzzer stats are reported to the user
    let monitor = SimpleMonitor::new(|s| println!("{s}"));

    // The event manager handle the various events generated during the fuzzing loop
    // such as the notification of the addition of a new item to the corpus
    let mut mgr = SimpleEventManager::new(monitor);

    // A queue policy to get testcasess from the corpus
    let scheduler = QueueScheduler::new();

    // A fuzzer with feedbacks and a corpus scheduler
    let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

    // Create the executor for an in-process function with just one observer
    let mut executor = InProcessExecutor::new(
        &mut harness,
        tuple_list!(observer),
        &mut fuzzer,
        &mut state,
        &mut mgr,
    )
    .expect("Failed to create the Executor");

    let mut generator = NautilusGenerator::new(&context);

    // Generate 8 initial inputs
    state
        .generate_initial_inputs_forced(&mut fuzzer, &mut executor, &mut generator, &mut mgr, 8)
        .expect("Failed to generate the initial corpus");

    // Setup a mutational stage with a basic bytes mutator
    let mutator = StdScheduledMutator::with_max_stack_pow(
        tuple_list!(
            NautilusRandomMutator::new(&context),
            NautilusRandomMutator::new(&context),
            NautilusRandomMutator::new(&context),
            NautilusRandomMutator::new(&context),
            NautilusRandomMutator::new(&context),
            NautilusRandomMutator::new(&context),
            NautilusRecursionMutator::new(&context),
            NautilusSpliceMutator::new(&context),
            NautilusSpliceMutator::new(&context),
            NautilusSpliceMutator::new(&context),
        ),
        2,
    );
    let mut stages = tuple_list!(StdMutationalStage::new(mutator));

    fuzzer
        .fuzz_loop(&mut stages, &mut executor, &mut state, &mut mgr)
        .expect("Error in the fuzzing loop");
}
