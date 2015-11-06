extern crate rand;

use rand::Rng;
use std::io;
use std::cell::Cell;

// Die
struct Die {
    sides: i32,
    last_value: Cell<i32>
}

impl Die {
    fn new(sides: i32) -> Die {
        Die {
            sides: sides,
            last_value: Cell::new(0)
        }
    }

    fn roll(self) -> i32 {
        self.last_value.set(rand::thread_rng().gen_range(1, self.sides + 1));
        self.last_value.get()
    }
}

// Input helpers
fn stdin_get_i32() -> i32 {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to convert to number \"{}\"", input)
    };

    input
}

fn main() {
    println!("Number of sides per die?");
    let die_sides = stdin_get_i32();

    println!("Number of dice?");
    let num_of_dice = stdin_get_i32();

    let dice: Vec<Die> = (0..num_of_dice).map(|_| {
        Die::new(die_sides)
    }).collect();

    println!("=== New Roll ===");

    let mut rollable_die_indices: Vec<i32> = (0..num_of_dice).map(|n| { n }).collect();
    let mut fixed_die_indices: Vec<i32> = Vec::new();

    for i in (0..num_of_dice) {
        if rollable_die_indices.contains(&i) {
            print!("O ");
        } else {
            print!("  ");
        }

        print!("{}. {}\n", i, &dice[i as usize].clone().roll());
    }
}
