# RustBox
Playing with Rust    
- [Rust Docs](https://www.rust-lang.org/learn)
- [Rust Book](https://doc.rust-lang.org/book/)

----
Getting Started

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
cargo new hello_cargo
cd hello_cargo
```

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
