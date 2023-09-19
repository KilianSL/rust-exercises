use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::num::ParseIntError;

fn get_user_input() -> Result<u32, ParseIntError> {
    println!("Input a guess:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    guess.trim().parse()
}

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        let guess = match get_user_input() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low :("),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
            Ordering::Greater => println!("Too high :("),
        }
    }
}
