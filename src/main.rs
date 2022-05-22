use std::io;
use std::ops::{RangeInclusive};
use rand::{Rng, thread_rng};

const MAGIC_NUMBER_RANGE: RangeInclusive<i32> = 1..=50;

fn main() {
    let magic_number = generate_magic_number();

    println!("Welcome!");
    println!("Please enter your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading input");

    println!("You guessed {}", guess);

    println!("The magic number is {}", magic_number);
}

fn generate_magic_number() -> i32 {
    let mut rng = thread_rng();

    let magic_number = rng.gen_range(MAGIC_NUMBER_RANGE);

    magic_number
}