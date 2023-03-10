/*  A tuple is a collection of values of different types.

Tuples are constructed using parentheses and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the types of its members.

Functions can use tuples to return multiple values, as tuples can hold any number of values. */


// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 'let' can be used to bind the members of a tuple to variables
    let (int_param, bool_param) = pair;
}



// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32)

fn main() {
    // A tuple with several different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                    -1i8,, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64,
                    'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple-0);
}