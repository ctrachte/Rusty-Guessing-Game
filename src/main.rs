use std::io;
use rand::Rng;
use std::cmp::Ordering;

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
        
    let guess1: u32 = guess1
        .trim()
        .parse()
        .expect("Please type a number!");
    let guess2: u32 = guess2
        .trim()
        .parse()
        .expect("Please type a number!");
        
    println!("You first guessed: {} then guessed: {}", guess1, guess2);

    match guess1.cmp(&secret_number) {
        Ordering::Less => println!("{} is too small!", guess1),
        Ordering::Greater => println!("{} is too big!", guess1),
        Ordering::Equal => println!("{} is correct! You win!", guess1),
    }

    match guess2.cmp(&secret_number) {
        Ordering::Less => println!("{} is too small!", guess2),
        Ordering::Greater => println!("{} is too big!", guess2),
        Ordering::Equal => println!("{} is correct! You win!", guess2),
    }

}