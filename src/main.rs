use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    // Generating random number
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Shhh... the answer is {random_number}\n");

    // Game introduction
    println!("Welcome to the \"Guessing Game\", game!");
    println!("In order to achieve the victory, you need to guess a number between 1 and 100!");
    println!();

    // Repeatedly Getting input
    loop {
        println!("So, your guess is...");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to get user guess.");

        // Shadowing guess into u32 data type.
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("\nInvalid input. Try again.");
                continue;
            }
        };

        // Comparing guess        
        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!();
                println!("Congratulations! You won the game!");
                
                break;
            },
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Less => println!("Too small!\n")
        }
    }
}