use std::io;

fn main() {
    // `println!()` is a macro that can receive a variable number of arguments.
    // To learn more: `cargo install cargo-expand`, then `run cargo expand`.
    println!("Number: {}, String: {}", 100, "abcde");

    let mut input = String::new();
    // The `mut` keyword makes our new variable mutable (changeable).
    // The `input` variable is the owner of our `String` [1].
    // The `String` lives on the heap, because its size is not known at compile time.
    // The stack contains a pointer to the `String` and some additional metadata.

    io::stdin().read_line(&mut input);
    println!("Input: {}", input);
    let mut mars_weight: f32 = calculate_weight_on_mars(90.0); // Compiler infers `weight: f32` from the function argument.
    println!(
        "Weight on Mars: {} kg",
        calculate_weight_on_mars(mars_weight)
    );
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711 // Don't add a semicolon to your return expression!
}

// 1. Each value in Rust is owned by a variable.
// 2. When the owner goes out of scope, the value will be deallocated. (see RAII in C++)
// 3. There can only be ONE owner at a time.
