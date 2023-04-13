use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game !");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("\nYour Guess : ");
        let mut user_guess = String::new();
        io::stdin().read_line(&mut user_guess).expect("err");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
