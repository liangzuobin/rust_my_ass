use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    guess_number()
}

fn guess_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Result<u32, std::num::ParseIntError> = parse_u32(guess);

        if !guess.is_ok() {
            println!("Please type a number");
            continue;
        }

        let guess: u32 = guess.unwrap();
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

fn parse_u32(s: String) -> Result<u32, std::num::ParseIntError> {
    let i: u32 = s.trim().parse()?;
    Ok(i)
}
