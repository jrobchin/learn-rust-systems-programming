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

## Compiling for Release
To compile for release, pass the `--release` flag to `cargo build` or `cargo run`.

## cargo vs. rustc
- **rustc** is the Rust compiler
- **cargo** is a more general Rust tool which provides a simple interface to Rust tasks
    - `cargo build` invokes **rustc** on our behalf
- The `--verbose` flag will print debugging information and show what is being passed to **rustc**

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

### Macros
Macros return code instead of values and are often used to simplify common patterns.

## What is Rust?
- The distinguishing feature is that it prevents invalid data access at compile time
- [Research](https://msrc-blog.microsoft.com/2019/07/18/we-need-a-safer-systems-programming-language/) has shown that ~70% of serious security bugs are related to invalid data access
- Guarentees memory safety without imposing run time costs
- Labelled as a systems programming language

### Rust Safety
Rust programs are free from:
- Danging pointers
- Data races
- Buffer overflow
- Iterator invalidation

### Overarching Features of Rust
#### Performance
- No garbage collector
- Pushes burden on the compiler
  
#### Concurrency
- No GIL to constrain a thread's speed
- "Fearless concurrency"

#### Memory efficiency
- Fixed-size structures, byte level management
- High-level constructs incur minimal run-time overhead

## Downsides of Rust
- Cyclic data structuers are difficult to model in Rust
- Compile times are slower
- Rust is strict meaning that programs only compile if everything is just right
- The language is large

## Best Fits for Rust
- Command-line utilities
- Data processing
- Extending applications
- Resource contrained environments
- Server-side applications
- Desktop applications
- Desktop
- Mobile
- Web (WASM)
- Systems programming