fn main() {
    println!("Number: {}, String: {}", 100, "abcde"); // `println!()` is a macro that can receive a variable number of arguments.
                                                      // To learn more: `cargo install cargo-expand`, then `run cargo expand`.

    println!("Weight on Mars: {} kg", calculate_weight_on_mars(100.0));
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) & 3.711 // No semicolon on return expression!
}
