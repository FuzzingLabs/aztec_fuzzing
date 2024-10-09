# AZTEC_FUZZING

## Description
Aztec_fuzzing lists various tools for fuzzing the Noir language compiler.
The goal is to randomly generate Noir code and then check the compiler's behavior to detect issues.

- **noir_smith** is the tool used to randomly generate Noir code. For a detailed description, please refer to `./noir_fuzzers/noir_smith/README.md`.

- **noir_fuzzers** Contain binaries used to fuzz the Noir compiler. For a detailed description, please refer to `noir_fuzzers/README.md`.

- **noir-minimizer** is a crash minimizer for the Noir compiler. It takes a crash files as input and tries to minimise it. For a detailed description, please refer to `noir_minimizer/README.md`.
