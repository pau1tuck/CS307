// * CONTROL FLOW

fn main() {
    let number = 4;

    // Blocks of code associated with the conditions in if expressions are sometimes called arms.
    if number < 5 {
        println!("Condition is true");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Condition is false");
    }
}

// You cannot have mismatched variable types in an if statement:
fn mismatch() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}

fn infinite_loop() {
    loop {
        println!("again!");
    }
    // continue / break
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
                break;
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
