use std::io;
use rand::Rng;
fn main()
{
    println!("Guess a number");
    let secret_number=rand::thread_rng().gen_range(1..=100);
    println!("The Secret number is {secret_number}");
    println!("Please Enter your guess");
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Error Reading the line ");
    println!("You guessed: {guess}");
}