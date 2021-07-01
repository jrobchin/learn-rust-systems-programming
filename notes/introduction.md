# Introduction

## Advocating for Rust at Work
- Security, confidence, bug identification - see: `cargo fuzz`
- Identifies existing bugs in code that is being rewritten
- Statically linked binaries

## Installing Rust
Rust is typically installed from [rustup.rs](https://rustup.rs). I also installed the Rust VSCode extension.

## Hello, world!
1. Install Rust
2. `$ cargo new hello`
3. `$ cd hello`
4. `$ cargo run`

## Cargo
- Build system and package manager
- It knows how to compile code and install dependencies
- Structures new projects the same for each one it creates
    - *Cargo.toml* describes the project's metadata
    - Source code goes in *src*
- `cargo run` compiles and then runs the source code

## Rust Features
### Unicode
Unicode is supported out of the box.

### iter()
Many types (e.g. arrays) have an `.iter()` method that returns an iterator.

### &
& (ampersand) borrows region for read-only access.

### Method Syntax
Rust is **not** object-oriented but it has a method syntax.

### Higher-order Programming
Functions can accept and return functions.

### Type Annotations
Relatively rare but sometimes required.

### Conditional Compilation
`cfg!()` checks compilation configuration at compile time and can be used to conditionally compile code.

### Implicit Returns
Rust provides a `return` keywork but it's usually omitted from code.