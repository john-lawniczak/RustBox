# RustBox
Playing with Rust    
- [Rust Docs](https://www.rust-lang.org/learn)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Standard Library](https://doc.rust-lang.org/std/)
- [Crates.io](https://crates.io/)

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024)

_Rust always wants to minimize unexpected updates to data._

_
----
Getting Started

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
-----
Check Version
```
rustc --version
cargo --version
```
Cargo - official package manager.
```
cargo new myproject
cd myproject
cargo build
```
`cargo check` - checks if code compiles without compiling    
`executable` - is in target/debug

`cargo install cargo-script` - llows you to quickly execute standalone Rust scripts without needing a full Cargo project structure    
`cargo script src/[filename].rs` - runs a specific Rust script ([filename].rs) located in the src folder, without requiring a Cargo.toml or the full project setup

`rustc` - Compiles individual Rust source files into executable binaries.
`cargo` - Manages Rust projects, builds, and runs code, handles dependencies.    
`cargo new` - Creates a new Rust project (binary or library).    
`cargo build`- Compiles the project.    
`cargo run` - Compiles and runs the project.    
 `cargo run -q` - Runs the project without displaying the build output (quiet mode).    
 `cargo test`  - Runs the project's tests.      
 `cargo clean` - Removes build artifacts (the `target/` directory).        
 `cargo update` - Updates project dependencies to the latest versions.    
 `cargo check` - Checks the code for errors without compiling.    
 `cargo doc` - Generates documentation for the project.    
`cargo publish` - Publishes the crate to crates.io.    
 `cargo install` - installs a binary or tool from crates.io.    
 `rustup` - manages Rust toolchains and versions.    
`rustup component add` - Adds additional components (e.g., Clippy, Rustfmt).    
 `rustup target add` - Adds a target architecture for cross-compilation.    
`rustdoc` - Generates documentation from Rust source comments.    
 `cargo bench` - Runs benchmark tests (requires nightly toolchain).    
 `cargo fix` - Automatically fixes code for compatibility with the latest Rust edition.    
 `cargo uninstall` - Uninstalls a previously installed binary.    
`cargo new --vcs` - Initializes a new project with version control (e.g., Git).    


-----
# Memory Allocation: Ownership, Borrowing, and Lifetimes
[Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)    
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

-----
# Data Types
Statically typed.

Scalar Types: Represent a single value:

- **Signed Integers**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (no decimal places)
- **Unsigned Integers**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (no decimal places)

Floating-Point Types:
- `f32`: 32-bit floating point
- `f64`: 64-bit floating point (default)

- `bool`: Values can be `true` or `false`.
- `char`: Represents a single Unicode scalar value.

Compound Types: group multiple values into one type.

- Tuples have a fixed length: `(value1, value2, ...)`.
- Arrays have a fixed length: `[element; length]`.


Rust also provides ways to define custom data types using `struct` and `enum`.
- Used to create custom data types.
  
  ```rust
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }

### Explicit Return

Explicitly return a value using the `return` keyword, followed by the value to return. This is optional because Rust allows returning the result of the last expression without using `return`.

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;  // Explicitly returns the sum of a and b
    // IMPLICIT return
    // a + b   -- as long as there is NO SEMICOLON
}
```

-----

# Fuzzing

Fuzzing helps discover edge cases and bugs by throwing lots of randomized input at your functions.

## 🔍 `cargo-fuzz` (most common fuzzing tool)

- Install: `cargo install cargo-fuzz`
- Initialize: `cargo fuzz init`
- Add a fuzz target: `cargo fuzz add my_fuzz_target`
- Run: `cargo fuzz run my_fuzz_target`

```rust

Example target (`fuzz_targets/my_fuzz_target.rs`):
```rust
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Call the function you're testing here
    let _ = my_function_to_fuzz(data);
});
```


