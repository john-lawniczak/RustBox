# RustBox
Playing with Rust    
- [Rust Docs](https://www.rust-lang.org/learn)
- [Rust Book](https://doc.rust-lang.org/book/)

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

Sample Guessing Game
```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```
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
