// * FUNCTIONS, STATEMENTS, EXPRESSIONS

fn main() {
    println!("Hello, world!");
    // Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.
    yet_another_function(5, 'h');
    // "The value of x is: 5

    // An expression:
    let y = {
        let z = 3;
        // Expressions do not include ending semicolons.
        // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
        z + 1
    };

    println!("The value of y is: {y}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

// Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.
fn another_function() {
    println!("Another function.");
}

// Technically, the concrete values are called arguments, but in casual conversation, people tend to use the words parameter and argument interchangeably for either the variables in a function’s definition or the concrete values passed in when you call a function.
// In function signatures, you must declare the type of each parameter.
fn yet_another_function(x: i32, unit: char) {
    println!("The value of x is: {x}");
    println!("The measurement is: {x}{unit_label}");
}

// Functions can return values to the code that calls them.
// We don’t name return values, but we must declare their type after an arrow (->).
fn add_one(x: i32) -> i32 {
    x + 1 // Notice the lack of a semicolon. This is important.
}
