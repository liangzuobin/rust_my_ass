use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    let parse_u32 = |s: String| -> Result<u32, std::num::ParseIntError> {
        let i: u32 = s.trim().parse()?;
        Ok(i)
    };

    let mut guess;

    loop {
        println!("Please input your guess.");
        guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match parse_u32(guess) {
            Ok(guess) => match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("Too big"),
                Ordering::Equal => {
                    println!("You win");
                    break;
                }
            },
            Err(_) => {
                println!("Please type a number.");
                continue;
            }
        }
    }
}
