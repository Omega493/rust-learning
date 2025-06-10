// Program to generate a random number and compare it with user input.

use std::{cmp::Ordering, io, io::Write};
// cmp::Ordering -> used for comparisions
//std::io::Write -> used for flusing out the output (line 17, 18)

use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::rng().random_range(1..=100);
    // Generating a random number. (1..=100) symbolizes 100 is included. Had it been (1..100), 100 would've been excluded.
    
    let mut count = 0;

    loop {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        // println!() flushes the output and adds a newline automatically. print!() doesn't. So we had to manually flush the output. This won't add a newline.

        let mut guess = String::new(); // Note: guess here is a string.

        //Reading user input.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Converting the "guess" variable into integer.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        count += 1;

        println!("You guessed: {guess}");

        // Comparision + display of number of tries user took.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("Number of guesses: {count}");
                break;
            }
        }
    }
}