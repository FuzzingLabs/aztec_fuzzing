### Noir Crash Minimizer

The noir minimizer takes a noir file which crash the compiler as input and tries to minimize it. 

#### Run the minimizer

```
cargo run -- -f ./corpus/crash-1.nr
```

Minimized files are stored in the `minimized_output` folder.