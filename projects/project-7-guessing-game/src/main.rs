use rand::Rng;
// The 'Rng trait' defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
use std::io;

fn main() {
    println!("Try to guess the number!");

    println!("Please enter your guess.");

    let mut guess = String::new();
    // An associated function is a function that’s implemented on a type, in this case String.

    io::stdin()
        .read_line(&mut guess)
        // read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value.
        .expect("Failed to read line");

    /* Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states.
    We call each possible state a 'variant'.
    The purpose of these Result types is to encode error-handling information. */

    println!("You guessed: {guess}");
}
