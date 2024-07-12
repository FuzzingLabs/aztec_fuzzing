# NOIR_FUZZERS

## Description
**noir_fuzzers** Contain binaries used to fuzz the Noir compiler with Noir codes randomly generated mostly with `noir_smith`. You can build and run them separately. A description of each binary is provided below:

- **basic_run:** This binary fuzzes the compiler by calling it with function calls. It displays the number of attempts and the number of crashes found. Any crashes are stored in the `crashes_found` directory along with the corresponding code and error messages.

- **basic_cmd_run:** Similar to `basic_run`, but the compiler is invoked via the command line.

- **hfuzz_run:** Utilizes Honggfuzz to fuzz the compiler. This binary uses function calls to invoke the compiler, enabling Honggfuzz to obtain coverage information.

- **hfuzz_cmd_run:** Similar to `hfuzz_run`, but the compiler is invoked via the command line, thus coverage information cannot be obtained.

- **hfuzz_run_parser:** Similar to `hfuzz_run`, but will only invoke the parser.

- **reproducer:** Takes a Honggfuzz workspace folder as a parameter and generate codes for the inputs that triggered unique crashes. The codes generated are placed in `crashes_found/` directory along with the corresponding error messages.

- **dharma_grammar_run:** Similar to `basic_run`, but the code is generated via call by command line to Dharma, utilizing the grammars found in the `dharma_grammars` folder.

## Typical Usage Flow

1. **Running `hfuzz_run`:**
   - It's recommended to start fuzzing with an existing corpus to improve coverage and effectiveness and bypass the part where honggfuzz understand that the data need to be bigger than MIN_DATA_LENGTH.
   - Run `hfuzz_run` using Honggfuzz:
     ```bash
     HFUZZ_RUN_ARGS="-n 4 -t 30 -i data_corpus" cargo hfuzz run hfuzz_run
     ```
     The arguments specify that we want 4 threads (-n 4), a timeout of 30 seconds (-t 30), and the specified corpus as input (-i data_corpus).
   - Honggfuzz will execute and monitor the compiler, using the inputs from the corpus and generating new inputs to maximize code coverage.

2. **Using `reproducer`:**
   - After running `hfuzz_run`, there might be several crashes found during the fuzzing session.
   - To analyze these crashes and generate the corresponding Noir codes that triggered them, use the `reproducer` binary:
     ```bash
     cargo run --bin reproducer hfuzz_workspace/hfuzz_run/
     ```
   - This will process the Honggfuzz workspace directory, extract the unique crashes, and place the generated Noir codes and error messages into the `crashes_found/` directory.

By following this workflow, you can effectively fuzz the Noir compiler, identify crashes, and obtain the specific inputs that caused those crashes for further analysis and debugging.

## Installation
To build the project, Rust must be installed on your system. Follow these steps:

1. Specify the version of Noir that you want to fuzz by updating the branch in the `Cargo.toml` dependencies. You need to install Noir if you want to use binaries that invoke the compiler via command line. To install Noir see [Noir Documentation](https://noir-lang.org/docs/getting_started/installation/).

2. Install Dharma if you want to use `dharma_grammar_run` and generate code with Dharma [Dharma repository](https://github.com/posidron/dharma)

3. Build the entire project using the following command:
```bash
cargo build
```

Alternatively, you can build a specific binary using a command like this:
```bash
cargo build --bin name_of_the_wanted_bin
```

## Usage
Run the desired binary using the following command:
```bash
cargo run --bin name_of_the_wanted_bin
```

If the binary utilizes Honggfuzz, use the following command instead:
```bash
cargo hfuzz run name_of_the_wanted_bin
```
And then you can use the reproducer like this:
```bash
cargo run --bin reproducer hfuzz_workspace/hfuzz_run/
```