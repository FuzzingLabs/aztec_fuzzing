# Noir Grammar fuzzing

Fuzzing using a grammar is not very efficient for now. But we are exploring more options.

## LibAFL Nautilus

Generate random noir using the grammar `noir.json` and tries to compile it. The fuzzer will run until the first crash.

You can start it using:

```bash
$ cargo run
```

You need to have the noir repository locally and put the path in the `Cargo.toml` file.

```toml
[package]
name = "libafl_nautilus"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
libafl = { version = "0.11.2", features = ["default", "nautilus"]}
libafl_bolts = "0.11.2"
# Noir dependencies
noirc_frontend = { path = "../noir/compiler/noirc_frontend" } // Here
noirc_driver = { path = "../noir/compiler/noirc_driver" } // Here
fm = { path = "../noir/compiler/fm" } // // Here
nargo_toml = { path = "../noir/tooling/nargo_toml" } // Here
nargo = { path = "../noir/tooling/nargo" } // Here

```

## Tree-Splicer

Since *Nautilus* was not really successful we tried generating code using (tree-splicer)[https://github.com/langston-barrett/tree-splicer]. It uses tree-sitter grammar to mutate a given code.

You can tried it using the following command:

```bash
$ ./target/release/tree-splicer-noir --on-parse-error ignore -c 10 -m 30 examples/main.nr
```

Generated files will be in the **tree-splicer.out** directory.