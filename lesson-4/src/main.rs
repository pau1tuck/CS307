// * CONTROL FLOW

fn main() {
    let number = 4;

    // Blocks of code associated with the conditions in if expressions are sometimes called arms.
    if number < 5 {
        println!("Condition is true");
    } else if number % 2 == 0 {
        println!("'number' is divisible by 2");
    } else {
        println!("Condition is false");
    }
}

// You cannot have mismatched variable types in an if statement:
fn mismatch() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of 'number' is: {number}");
}

fn infinite_loop() {
    loop {
        println!("again!");
    }
    // Use 'continue' or 'break'
}

fn looping_counter() {
    let mut counter: u8 = 0; // u8 allows 0 to 255

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn nested_loops() {
    let mut count = 0;
    'counting_up: loop {
        // We can give our loop a label
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // Exits the inner loop only
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    // Eliminates the need for unnecessary nested loops
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFT OFF!!!");
}

fn while_with_array() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]); // Access the array indices with a[index]

        index += 1;
    }
}

// A safer and more efficient method:
fn for_with_array() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
}
/* The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
Even in situations in which you want to run some code a certain number of times, as in while_with_array, most Rustaceans would use a for loop.  */

fn reverse_for_loop {
    for number in (1..4).rev() { // rev() method
        println!("{number}!");
    }
    println!("LIFT OFF!!!");
}
