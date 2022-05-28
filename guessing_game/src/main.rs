use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {

    println!("             The Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut guess = String::new();
    loop {
        // need to clear the string buffer to read number in subsequent iteration
        // or paring `guess` will panick with InvalidDigit error
        guess.clear();
        println!("Please input your guess!");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }
    }
}

