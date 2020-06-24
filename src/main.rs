use std::io;

fn main() {
    println!("Guess the number!");

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

    println!("You first guessed: {} then guessed {}", guess1, guess2);
}