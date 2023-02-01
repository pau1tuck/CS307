fn main() {
    // * IMMUTABILITY
    let _x = 5;
    println!("The value of x is: {_x}");
    _x = 6; // Here is the bug.
    println!("The value of x is: {_x}");

    // Although variables are immutable by default, you can make them mutable by adding mut in front of the variable name.
    let mut _y = 5;
    println!("The value of x is: {_y}");
    _y = 6;
    println!("The value of x is: {_y}");

    // * CONSTANTS
    /* Constant evaluation is the process of computing the result of expressions during compilation. Only a subset of all expressions can be evaluated at compile-time.
    There are a few differences between constants and variables. */
    // First, you aren’t allowed to use mut with constants.
    // Constants aren’t just immutable by default—they’re always immutable.

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // The type of the value must be annotated.
    // Rust’s naming convention for constants is to use all uppercase with underscores between words.

    /* Constants are valid for the entire time a program runs, within the scope in which they were declared. This property makes constants useful for values in your application domain that multiple parts of the program might need to know about, such as the maximum number of points any player of a game is allowed to earn, or the speed of light. */

    // * SHADOWING
    // You can declare a new variable with the same name as a previous variable.
    // Rustaceans say that the first variable is shadowed by the second.

    let z = 5;

    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of x in the inner scope is: {z}");
    }

    println!("The value of z is: {z}");

    // By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

    // Changing the variable type:
    let _spaces = "   ";
    let _spaces = _spaces.len();
    // Note that we can't do this with mut.

    // Shadowing thus spares us from having to come up with different variable names, such as spaces_str and spaces_num.
}
