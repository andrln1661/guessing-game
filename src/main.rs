use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("\x1b[36mPlease input you guess\x1b[0m");

        let mut guess = String::new(); //mutable like let in js

        io::stdin()
            .read_line(&mut guess)
            .expect("\x1b[31mFailed to read line\x1b[0m");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\x1b[33mYou guessed: {} \x1b[0m", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!(
                "{} !! ==> {}{}",
                "\x1b[31mToo small", "\x1b[34mNext attempt?", "\x1b[0m"
            ),
            Ordering::Greater => {
                println!(
                    "{} !! ==> {}{}",
                    "\x1b[31mToo big", "\x1b[34mNext attempt?", "\x1b[0m"
                )
            }
            Ordering::Equal => {
                println!("!! {} !!", "\x1b[32mYou won\x1b[0m");
                break;
            }
        }
    }
}
