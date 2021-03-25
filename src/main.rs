use regex::Regex;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-22"));

    // IO Package learning
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {}", guess);

    // Testing rand package
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret number is {}", secret_number);

    let guess: u32 = guess.trim().parse().expect("Please type a number");
    // Compare package
    println!("guess {}", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("Is equal"),
        Ordering::Greater => print!("Too large"),
    }

    loop {
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too smaller"),
            Ordering::Equal => {
                println!("Is equal");
                break;
            }
            Ordering::Greater => println!("Too larger"),
        }
    }
}
