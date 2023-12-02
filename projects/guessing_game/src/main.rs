use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please enter a number between 1 and 100");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    let guess: u32 = guess.trim().parse().expect("Please enter a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{guess} is too Low!"),
        Ordering::Greater => println!("{guess} is too High!"),
        Ordering::Equal => println!("We have a winner!"),
    }
}
