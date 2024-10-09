### Noir Crash Minimizer

The noir minimizer takes a noir file which crash the compiler as input and tries to minimize it. 

#### Requirements

[Nargo](https://noir-lang.org/docs/getting_started/installation/)

#### Run the minimizer

```
cargo run -- -f ./corpus/crash-1.nr
```

Minimized files are stored in the `minimized_output` folder. You can fin examples of crashes to minimize in the `minimized_output` folder.
