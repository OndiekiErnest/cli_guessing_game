use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
pub mod validation;

fn main() {
    // use colored strings from colored library
    println!("{}", "Guess Number Game!".bold().yellow());

    // guess only once; not for every loop
    let system_guess = rand::thread_rng().gen_range(1..=100);
    // number of tries starts at 1
    let mut num_of_attempts: u32 = 1;
    // display the random guess
    // println!("System guess: {system_guess}");
    println!("{}", "Type a number between 1 and 100".bold().yellow());

    loop {
        

        println!("{}", "Please input your guess.".bold().blue());

        // instantiate user_guess string buffer
        let mut user_guess = String::new();

        // use match to handle errors
        match io::stdin()
            // use mutable user_guess reference; doesn't take ownership
            .read_line(&mut user_guess) {
                Ok(guess) => guess,
                Err(_) => {
                    println!("{}", "Failed to read line!".bold().red());
                    continue;
                },
            };

        // convert user_guess to integer
        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("{}", "Only integers allowed!".bold().red());
                continue;
            },
        };

        let valid_guess = validation::Guess::new(user_guess);

        // println!("You guessed: {user_guess}");

        // use match to handle to diff. comparisons
        match system_guess.cmp(&valid_guess.value()) {
            Ordering::Equal => {
                println!("{} Attempts: {num_of_attempts}", "You won!".bold().green());
                break;
            },
            Ordering::Greater => {
                println!("{}", "Number too big!".bold().red());
                num_of_attempts += 1;
            },
            Ordering::Less => {
                println!("{}", "Number too low!".bold().red());
                num_of_attempts += 1;
            },
        }
    }

}