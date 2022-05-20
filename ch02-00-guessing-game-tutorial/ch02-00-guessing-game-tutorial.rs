use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    // println!("Hello, world!");

    /* In Rust, variables are immutable by default.
    To make a variable mutable, we add mut before the variable name: */
    // let apples = 5; // immutable
    // let mut bananas = 5; // mutable

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {secret_number}");

    let message = "Guess the number!\nAnother line goes here.";
    println!("{}", message);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        match io::stdin()
            .read_line(&mut guess) // The & indicates that this argument is a reference
        /*.expect("Failed to read line")*/ {
            Ok(_n) => { // help: if this is intentional, prefix it with an underscore: `_n`
                // println!("{n} bytes read");
                println!("You guessed: {}", guess);
            }
            Err(error) => println!("error: {error}"),
        }

        /* Rust allows us to shadow the previous value of guess with a new one.
        Shadowing lets us reuse the guess variable name rather than forcing us
        to create two unique variables, such as guess_str and guess for example. */
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}