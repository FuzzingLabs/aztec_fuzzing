# Overflowing u128 using FieldElement to_u128 function in FieldElement in generik_rk.rs in acir_field in ACVM

We (**@fuzzinglabs**) found we can have an u128 overflow using the to_128 function from FieldElement in MassaLabs making the library unsafe. 
The **acir::FieldElement::to_u128** function from **FieldElement** converts FieldElement into u128. For the case where the FieldElement value exceed the u128 max bound, the to_u128 function from FieldElement leeds to an overflow.
The library should use try_to_u128 over the to_u128 in all the cases, as it is the case for the u64 with the try_to_u64 function.

## Your Environment

- rustc 1.71.1 (eb26296b5 2023-08-03)
- Ubuntu 20.04

## Steps to reproduce

Download:

``` sh
git clone git@github.com:noir-lang/acvm.git
```

Testing program:

***main.rs***:

``` rust
use acir_field::FieldElement;

fn main() {
    println!("-------------------------------------------------------------");
    // Using the u128 max value
    println!("field_element value: {:?}", FieldElement::from(u128::MAX));
    println!("to_u128 output: {:?}", FieldElement::from(u128::MAX));
    println!("try_into_u128 output: {:?}", FieldElement::from(u128::MAX));

    println!("-------------------------------------------------------------");
    // Using the u128 max value + 1
    println!("field_element value: {:?}", FieldElement::try_from_str("340282366920938463463374607431768211456").unwrap());
    println!("to_u128 output: {:?}", FieldElement::try_from_str("340282366920938463463374607431768211456").unwrap().to_u128());
    println!("try_into_u128 output: {:?}", FieldElement::try_from_str("340282366920938463463374607431768211456").unwrap().try_into_u128());

    println!("-------------------------------------------------------------");
    // Using the u128 max value + 2
    println!("field_element value: {:?}", FieldElement::try_from_str("340282366920938463463374607431768211457").unwrap());
    println!("to_u128 output: {:?}", FieldElement::try_from_str("340282366920938463463374607431768211457").unwrap().to_u128());
    println!("try_into_u128 output: {:?}", FieldElement::try_from_str("340282366920938463463374607431768211457").unwrap().try_into_u128());

    println!("-------------------------------------------------------------");
}

```

***Cargo.toml***:

```
[package]
name = "attempt_to_negate_with_overflow"
version = "0.1.0"
edition = "2021"

[dependencies]
acir_field = { path = "../acvm/acir_field" }

```

Build and run:

``` sh
cargo build
cargo run
```

## Root cause

https://github.com/noir-lang/acvm/blob/2f503c4b444d68d4b8489eba78e95e6889382c2e/acir_field/src/generic_ark.rs#L227

## Detailed behavior (RUST_BACKTRACE=1)

``` sh
thread 'main' panicked at 'attempt to negate with overflow', /home/sebastien/Downloads/acvm/acir_field/src/generic_ark.rs:98:17
stack backtrace:
   0: rust_begin_unwind
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/std/src/panicking.rs:593:5
   1: core::panicking::panic_fmt
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/panicking.rs:67:14
   2: core::panicking::panic
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/panicking.rs:117:5
   3: <acir_field::generic_ark::FieldElement<F> as core::convert::From<i128>>::from
             at /home/sebastien/Downloads/acvm/acir_field/src/generic_ark.rs:98:17
   4: je_suis_un_test::main
             at ./src/main.rs:4:13
   5: core::ops::function::FnOnce::call_once
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```