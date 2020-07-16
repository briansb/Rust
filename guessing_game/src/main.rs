
// I/O library
use std::io;

fn main() {
    // println is a macro...trailing !
    println!("Guess the number!");

    println!("Please input your guess.");

    // binds the value String::new() (which is empty) to the 
    // mutable variable guess
    let mut guess = String::new();

    // from above, using stdin() from the io module
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

