use std::io;

fn main() {
    println!("Try to guess the number!");

    println!("Please enter your guess.");

    let mut guess = String::new();
    // An associated function is a function that’s implemented on a type, in this case String.

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
