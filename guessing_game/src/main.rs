use std::io; // need to bring the input/output libary to scope from
             // the standard library, known as std
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the Number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Variable that stores user input (mutable)

        io::stdin() // <-- this stdin func allows us to handle user input 
            .read_line(&mut guess) // read_line gets input from the user
            .expect("Failed to read line"); // &mut guess is the argument as
                                        // to tell it what string to store
                                        // in the user input 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println! ("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
