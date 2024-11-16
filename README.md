# RustBox
Playing with Rust    
- [Rust Docs](https://www.rust-lang.org/learn)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Standard Library](https://doc.rust-lang.org/std/)
- [Crates.io](https://crates.io/)

----
Getting Started

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```rust
fn main() {
    println!("Hello, world!");
}
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

### 1. `rustc`
   - Compiles individual Rust source files into executable binaries.

### 2. `cargo`
   - Manages Rust projects, builds, and runs code, handles dependencies.

### 3. `cargo new`
   - Creates a new Rust project (binary or library).

### 4. `cargo build`
   - Compiles the project.

### 5. `cargo run`
   - Compiles and runs the project.

### 6. `cargo run -q`
   - Runs the project without displaying the build output (quiet mode).

### 7. `cargo test`
   - Runs the project's tests.

### 8. `cargo clean`
   - Removes build artifacts (the `target/` directory).

### 9. `cargo update`
   - Updates project dependencies to the latest versions.

### 10. `cargo check`
   - Checks the code for errors without compiling.

### 11. `cargo doc`
   - Generates documentation for the project.

### 12. `cargo publish`
   - Publishes the crate to crates.io.

### 13. `cargo install`
   - Installs a binary or tool from crates.io.

### 14. `rustup`
   - Manages Rust toolchains and versions.

### 15. `rustup component add`
   - Adds additional components (e.g., Clippy, Rustfmt).

### 16. `rustup target add`
   - Adds a target architecture for cross-compilation.

### 17. `rustdoc`
   - Generates documentation from Rust source comments.

### 18. `cargo bench`
   - Runs benchmark tests (requires nightly toolchain).

### 19. `cargo fix`
   - Automatically fixes code for compatibility with the latest Rust edition.

### 20. `cargo uninstall`
   - Uninstalls a previously installed binary.

### 21. `cargo new --vcs`
   - Initializes a new project with version control (e.g., Git).


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

- **Signed Integers**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- **Unsigned Integers**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

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
