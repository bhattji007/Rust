use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main()
{
    println!("Guess a number");
    let secret_number=rand::thread_rng().gen_range(1..=100);
    // println!("The Secret number is {secret_number}");
    loop{
    println!("Please Enter your guess");
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Error Reading the line ");
    let guess: u32 = match  guess.trim().parse(){
        Ok(num)=>num ,
        Err(_)=> continue,
    };
    println!("You guessed: {guess}");
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Smaller oops!!"),
        Ordering::Greater => println!("Greater oops!!"),
        Ordering::Equal =>{
             println!("Exactly the number!!");
             break;
            }
    }

}
}