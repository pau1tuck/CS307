fn main() {
    let _x = 5;
    println!("The value of x is: {_x}");
    _x = 6; // Here is the bug.
    println!("The value of x is: {_x}");

    // Although variables are immutable by default, you can make them mutable by adding mut in front of the variable name.
    let mut _y = 5;
    println!("The value of x is: {_y}");
    _y = 6;
    println!("The value of x is: {_y}");

    // There are a few differences between constants and variables.
    // First, you aren’t allowed to use mut with constants.
    // Constants aren’t just immutable by default—they’re always immutable.

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // The type of the value must be annotated.
    // Rust’s naming convention for constants is to use all uppercase with underscores between words.
}
