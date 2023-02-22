// 6. Mutability

fn main() {
    println!("Number: {}, String: {}", 100, "abcde"); // `println!()` is a macro that can receive a variable number of arguments.
                                                      // To learn more: `cargo install cargo-expand`, then `run cargo expand`.
                                                      // The `mut` keyword makes our new variable mutable (changeable):
    let mut mars_weight: f32 = calculate_weight_on_mars(90.0); // Compiler infers `weight: f32` from the function argument.
    println!(
        "Weight on Mars: {} kg",
        calculate_weight_on_mars(mars_weight)
    );
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711 // Don't add a semicolon to your return expression!
}
