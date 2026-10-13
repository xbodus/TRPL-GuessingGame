use std::io;
use std::cmp::Ordering;
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::env;

fn main() {
    // Grab the current executable to restart the program
    let current_exe = env::current_exe().expect("Failed to get current exe path");

    // Generate a random number between 0-100
    let secret_number : i32 = rand::random_range(0..=100);

    println!("Guess the number!");
    println!("Input your guess between 0 and 100.");

    // main program loop
    loop {
        // User input store
        let mut guess: String = String::new();

        // Read user input from command line and store in user input store
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Restart condition
        if guess.trim().to_lowercase() == "restart" {
            let _ = Command::new(&current_exe).exec();
        }

        // Cast string to i32 for comparision
        let guess_int = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(e) => {
                println!("Failed to parse: {}", e);
                0 // Fallback value
            }
        };

        println!("Your guess: {guess}");

        // Input guard against out of range guesses
        if guess_int < 0 || guess_int > 100 {
            println!("Bad guess buster! You must be bad at instructions. Guess between 0-100");
            continue;
        }

        // Input comparision against secret number
        match guess_int.cmp(&secret_number) {
            Ordering::Less => println!("Try again. Guess too low."),
            Ordering::Greater => println!("Try again. Guess too high."),
            Ordering::Equal => {
                println!("Ho...How did you...? You smart bastard. You got it right! Congradulations.");
                println!("Want to play again? Just type restart.");
            }
        };
    }
}
