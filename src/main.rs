// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let maxx = 10_000;
    let secret: u16 = rand::thread_rng().gen_range(1..=maxx);
    println!("Guess a number 1-{maxx}: ");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("should have been a line");
        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("Yup!");
                break;
            },
            Ordering::Less => println!("Moar!"),
            Ordering::Greater => println!("Not that much!"),
        }
    }
}
