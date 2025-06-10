Documentation for personal reference use.

# Guess the Number Game in Rust

This is a simple Rust program that generates a random number and asks the user to guess it. The program provides feedback on whether the guess is too high or too low, and keeps track of the number of attempts.

## Features

- Generates a random number between 1 and 100 (inclusive)
- Reads user input and validates it
- Provides feedback on each guess ("Too small!" or "Too big!")
- Tracks and displays the number of guesses when the user wins
- Handles non-numeric input gracefully (just asks for input again)

## Code Explanation

### Dependencies

```rust
use std::{cmp::Ordering, io, io::Write};
use rand::Rng;
```

- `std::cmp::Ordering` - Used for comparisons between the guess and secret number
- `std::io` - Used for input/output operations
- `std::io::Write` - Specifically used for flushing output (important for `print!` macro)
- `rand::Rng` - Used for random number generation

### Main Function

1. **Initialization**:
   ```rust
   let secret_number = rand::rng().random_range(1..=100);
   let mut count = 0;
   ```
   - Generates a random number between 1 and 100 (inclusive)
   - Initializes a counter for guesses

2. **Game Loop**:
   ```rust
   loop {
       print!("Enter your guess: ");
       io::stdout().flush().unwrap();
    // --snip--
    }
   ```
   - Starts an infinite loop (broken when user guesses correctly)
   - Prints prompt without newline, requires manual flush

3. **Reading Input**:
   ```rust
   let mut guess = String::new();
   io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read line");
   ```
   - Creates a mutable string to store user input
   - Reads line from standard input

4. **Input Conversion**:
   ```rust
   let guess: u32 = match guess.trim().parse() {
       Ok(num) => num,
       Err(_) => continue,
   };
   ```
   - Converts string to unsigned 32-bit integer
   - Uses pattern matching to handle errors (continues loop if input isn't a number)

5. **Comparison Logic**:
   ```rust
   count += 1;
   match guess.cmp(&secret_number) {
       Ordering::Less => println!("Too small!"),
       Ordering::Greater => println!("Too big!"),
       Ordering::Equal => {
           println!("You win!");
           println!("Number of guesses: {count}");
           break;
       }
   }
   ```
   - Increments guess counter
   - Compares guess to secret number using pattern matching
   - Provides appropriate feedback
   - Breaks loop and displays win message when correct

## Key Rust Concepts Learned

- Variable mutability (`mut`)
- Error handling with `match` and `Result` type
- Type conversion (string to integer)
- Infinite loops with `loop` and controlled exit with `break`
- Standard input/output operations
- Random number generation
- Pattern matching with `Ordering` enum
