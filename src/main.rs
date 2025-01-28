use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please enter your guess:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to Read !!!");

    println!("You Guess {guess}");

    let guess:u32=guess.trim().parse().expect("NNNNO");

    match guess.cmp(&secret_number){
        Ordering::Equal => println!("You Won !!!"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Less => println!("Too Low56")

    }
    }
}
