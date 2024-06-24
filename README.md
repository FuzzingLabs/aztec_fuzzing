# NOIR_GENERATOR

## Description
The NOIR_GENERATOR tool is developed to identify issues within the Noir compiler by generating random valid programs and attempting to compile them. There are five different binaries included:

- **basic_run:** This binary fuzzes the compiler by calling it with function calls. It displays the number of attempts and the number of crashes found. Any crashes are stored in the `crashes_found` directory along with the corresponding code and error messages.

- **basic_cmd_run:** Similar to `basic_run`, but the compiler is invoked via the command line.

- **hfuzz_run:** Utilizes Honggfuzz to fuzz the compiler. This binary uses function calls to invoke the compiler, enabling Honggfuzz to obtain coverage information.

- **hfuzz_cmd_run:** Similar to `hfuzz_run`, but the compiler is invoked via the command line, thus coverage information cannot be obtained.

- **hfuzz_run_parser:** Similar to `hfuzz_run`, but will only invoke the parser.

- **reproducer:** Takes a Honggfuzz workspace folder as a parameter and generate codes for the inputs that triggered unique crashes. The codes generated are placed in `crashes_found/` directory along with the corresponding error messages.

- **dharma_grammar_run:** Similar to `basic_run`, but the code is generated via call by command line to Dharma, utilizing the grammars found in the `dharma_grammars` folder.

## Installation
To build the project, Rust must be installed on your system. Follow these steps:

1. Place the version of Noir you want to fuzz alongside noir_generator. You can find the version at [Noir's GitHub repository](https://github.com/noir-lang/noir). You can just install noir if you simply want to use binaries that invoke the compiler via command line.

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