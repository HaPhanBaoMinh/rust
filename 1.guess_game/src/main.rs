use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("---guess number---");

    loop {
        println!("Input your guess number");

        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1, 101);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        println!("Secret Number: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
