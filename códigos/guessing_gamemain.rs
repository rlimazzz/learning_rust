use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        println!("GUESSING GAME");
        println!("INPUT YOUR GUESS:");

        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1, 101);

        io::stdin().read_line(&mut guess)
            .expect("FAILED TO READ LINE");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("YOU GUESSED {}", guess);
        println!("THE SECRET NUMBER IS: {}", secret_number);
        match guess.cmp(&secret_number){
            Ordering::Less => println!("TOO SMALL!"),
            Ordering::Equal => {
                println!("YOU CHOOSE THE RIGHT ONE!");
                break;
            }
            Ordering::Greater => println!("TOO HIGH"),
        }
    }
}
