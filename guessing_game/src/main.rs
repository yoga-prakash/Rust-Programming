use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number game!");

    let mut guessed_number = String::new();

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        guessed_number.clear();

        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Please input a valid number");

        let guessed_string_to_number: i32 = match guessed_number.trim().parse() {
            Ok(parsed_number) => parsed_number,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("Secret number: {secret_number}");

        match guessed_string_to_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        };
    }
}
