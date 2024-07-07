# AZTEC_FUZZING

## Description
Aztec_fuzzing lists various tools for fuzzing the Noir language compiler.
The goal is to randomly generate Noir code and then check the compiler's behavior to detect issues.

- **noir_smith** is the tool used to randomly generate Noir code. You can use it by calling the `generate_code` function. The randomness is based on an array provided as a parameter to this function. The generator will pick all necessary data from this array.

- **noir_fuzzers** Contain binaries used to fuzz the Noir compiler. For a detailed description of each of these binaries, please refer to `noir_fuzzers/README.md`.