use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number between 0 and 100...");
    let secret_number = rand::thread_rng().gen_range(0..=100);

    println!("Input your guess: ");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number! Please enter a valid number");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess was too small."),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Guess was too large."),
        }
    }

    println!("The secret number was: {secret_number}");
}
