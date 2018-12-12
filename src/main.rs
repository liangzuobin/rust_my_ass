use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let min = 1;
    let max = 100;
    let secret_number = rand::thread_rng().gen_range(min, max);
    // println!("The secret number is: {}", secret_number);
    println!("电脑：猜猜我的秘密数字[{} {}]", min, max);

    let parse_u32 = |s: String| -> Result<u32, std::num::ParseIntError> {
        let i: u32 = s.trim().parse()?;
        Ok(i)
    };

    let mut guess;

    loop {
        guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match parse_u32(guess) {
            Ok(guess) => match guess.cmp(&secret_number) {
                Ordering::Less => println!("电脑：小了。"),
                Ordering::Greater => println!("电脑：大了。"),
                Ordering::Equal => {
                    println!("电脑：恭喜你，猜对了。");
                    break;
                }
            },
            Err(_) => {
                println!("电脑：请输入数字。");
                continue;
            }
        }
    }
}
