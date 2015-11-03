extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number_lower = 1;
    let secret_numner_upper = 101;

    println!("Guess a number between {} - {}", secret_number_lower, secret_numner_upper - 1);

    let secret_number = rand::thread_rng().gen_range(secret_number_lower, secret_numner_upper);

    loop {
        println!("Input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Error reading line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Just right!");
                break;
            }
        }
    }
}