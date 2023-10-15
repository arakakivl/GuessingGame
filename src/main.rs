use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Generating random number.
    let secret = rand::thread_rng().gen_range(0..=100);
    
    println!("Shh... secret is {secret}.");
    println!();

    // Game presentation.
    println!("Welcome to the \"Guessing Game\", game.");
    
    print!("Tell me your name: ");
    io::stdout().flush().expect("Failed to flush.");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line.");
    
    println!();

    let name = name.trim();
    println!("You're welcome, {name}.");

    // Getting answer
    loop {
        println!("What is your answer?");
        
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to get user answer.");

        let answer: u32 = answer.trim().parse().expect("Failed to parse user answer.");
        println!();

        match answer.cmp(&secret) {
            Ordering::Equal => {
                println!("You won!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too low!")
        }

        println!();
    }
}
