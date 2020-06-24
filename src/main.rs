use std::io;
use rand::Rng;

fn main() {
    println!("Guess a number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please input your first guess: ");

    let mut guess1 = String::new();
    let mut guess2 = String::new();

    io::stdin()
        .read_line(&mut guess1)
        .expect("Failed to read your input!");

    println!("Please input your second guess: ");

    io::stdin()
        .read_line(&mut guess2)
        .expect("Failed to read your input!");

    println!("You first guessed: {} then guessed: {}", guess1, guess2);
}