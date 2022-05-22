use std::io;

fn main() {
    println!("Welcome!");

    println!("Please enter your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading input");

    println!("You guessed {}", guess);
}