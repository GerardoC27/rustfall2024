use std::io;

fn main() {
    // Hard-coded secret number
    let secret_number = 42;

    println!("Welcome to the number guessing game!");
    println!("Guess the secret number between 1 and 100.");

    // Variable to track the number of guesses
    let mut guess_count = 0;

    loop {
        // Simulate user input by setting a mutable guess variable
        let mut guess = String::new();

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        guess_count += 1;

        // Check the guess using the check_guess function
        match check_guess(guess, secret_number) {
            0 => {
                println!("You win!");
                break;
            }
            1 => println!("Too big!"),
            -1 => println!("Too small!"),
            _ => unreachable!(),
        }
    }

    // Print the number of guesses it took
    println!("It took you {} guesses.", guess_count);
}

// Function to check the guess
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}
