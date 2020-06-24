use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut guess = String::new();

    loop {
        println!("Please input your guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your input!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} is too small!", guess),
            Ordering::Greater => println!("{} is too big!", guess),
            Ordering::Equal => {
                println!("{} is correct! You win!", guess),
                break;
            }
        }

    }

}