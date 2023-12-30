use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("---* guessing the number *---");

    let secret_num = rand::thread_rng().gen_range(1..=10);
    // println!("the secret_num is: {secret_num}");

    loop {
        println!("please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("invalid input. ");
                continue;
            }
        };

        println!("you guessed: {guess}");

        // compare the result
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        };
    }
}
