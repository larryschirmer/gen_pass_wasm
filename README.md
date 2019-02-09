# Generate Password

A web assembly module written in rust

## How to use

module takes four arguments:

- number of lowercase letters
- number of uppercase letters
- number of numbers
- number of symbols

and outputs a random shuffled string

## Build Commands

`cargo build --target wasm32-unknown-unknown --release`

`wasm-bindgen target/wasm32-unknown-unknown/release/gen_pass.wasm --out-dir pkg`
