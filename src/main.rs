use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the game!!");

    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Please input your guess!!");

        println!("Secret number {}", secret_number);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter the valid input...");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("you won!");
                break;
            }
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
        }

        println!("You Guessed {}: ", guess);
    }
}
