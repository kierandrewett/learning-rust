use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please enter your guess");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("Your guess was: {}", guess);
    
        match guess.cmp(&secret) {
            Ordering::Less => println!("That guess was too small."),
            Ordering::Greater => println!("That guess was too big."),
            Ordering::Equal => {
                println!("You got it correct!");
                break;
            }
        }
    }
}