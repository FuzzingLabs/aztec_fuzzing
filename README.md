# NOIR_GENERATOR

## Description
The NOIR_GENERATOR tool is developed to identify issues within the Noir compiler by generating random valid programs and attempting to compile them. There are five different binaries included:

- **basic_run:** This binary fuzzes the compiler by calling it with function calls. It displays the number of attempts and the number of crashes found. Any crashes are stored in the `crashes_found` directory along with the corresponding code and error messages.

- **basic_cmd_run:** Similar to `basic_run`, but the compiler is invoked via the command line.

- **hfuzz_run:** Utilizes Honggfuzz to fuzz the compiler. This binary uses function calls to invoke the compiler, enabling Honggfuzz to obtain coverage information. However, it consumes significant amounts of RAM.

- **hfuzz_cmd_run:** Similar to `hfuzz_run`, but the compiler is invoked via the command line, thus coverage information cannot be obtained.

- **reproducer:** Takes a Honggfuzz data file as input to reproduce the code and the error. The code will be written in `noir_project/src/main.nr` and the error will be printed.

## Installation
To build the project, Rust must be installed on your system. Follow these steps:

1. Replace the `nargo_cli` file in `noir/tooling/` with the `nargo_cli` file located at the root of the project.

2. Build the entire project using the following command:
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
