use std::cmp::Ordering;
use std::io;
use std::ops::{RangeInclusive};
use rand::{Rng, thread_rng};

const MAGIC_NUMBER_RANGE: RangeInclusive<i32> = 1..=50;

fn main() {
    let magic_number = generate_magic_number();

    println!("Welcome!");

    let mut game_finished = false;

    while !game_finished {
        println!("Please enter your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        let guess: i32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("You must enter a number!");
                continue;
            }
        };

        match guess.cmp(&magic_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("You win!");
                game_finished = true;
            },
            Ordering::Greater => println!("Too high!"),
        }

        println!();
    }
}

fn generate_magic_number() -> i32 {
    let mut rng = thread_rng();

    let magic_number = rng.gen_range(MAGIC_NUMBER_RANGE);

    magic_number
}