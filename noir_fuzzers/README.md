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