use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    println!("...");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Trim the input and attempt to parse it as an i32
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    println!("You guessed: {guess}");
}
