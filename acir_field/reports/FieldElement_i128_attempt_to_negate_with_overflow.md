# Crash when converting i128 negative value into FieldElement in generik_rk.rs in acir_field in ACVM

We (**@fuzzinglabs**) triggered an *attempt to negate with overflow* in MassaLabs making the library to crash. 
The **acir::FieldElement::from** function from **FieldElement** converts i128 into FieldElement. For the case the i128 value provided by the user is negative, the function attempt to negate the value without checking the i128 bounds.
The i128 bounds are [i128::MIN == -170141183460469231731687303715884105728 ; i128::MAX == 170141183460469231731687303715884105727].
If we put the valid i128 value i128::MIN as an argument into the function, the code will try to negate this value ```a = -a;```, this means we will have a value out of the i128 bounds because **-**i128::MIN == i128::MAX + 1.
The library should check this case before attemping to negate a negative i128.

```sh
thread 'main' panicked at 'attempt to negate with overflow', /home/sebastien/Downloads/acvm/acir_field/src/generic_ark.rs:98:17
```

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
use acir::FieldElement;

fn main() {
    let _ = FieldElement::from(i128::MIN);
}


```

***Cargo.toml***:

```
[package]
name = "attempt_to_negate_with_overflow"
version = "0.1.0"
edition = "2021"

[dependencies]
acir = { path = "../acvm/acir_field" }

```

Build and run:

``` sh
cargo build
cargo run
```

## Root cause

https://github.com/noir-lang/acvm/blob/2f503c4b444d68d4b8489eba78e95e6889382c2e/acir_field/src/generic_ark.rs#L98

## Detailed behavior (RUST_BACKTRACE=1)

``` sh
-------------------------------------------------------------
field_element value: 340282366920938463463374607431768211455
to_u128 output: 340282366920938463463374607431768211455
try_into_u128 output: 340282366920938463463374607431768211455
-------------------------------------------------------------
field_element value: 2¹²⁸
to_u128 output: 0
try_into_u128 output: None
-------------------------------------------------------------
field_element value: 340282366920938463463374607431768211457
to_u128 output: 1
try_into_u128 output: None
-------------------------------------------------------------
```