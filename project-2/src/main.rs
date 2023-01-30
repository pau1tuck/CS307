fn main() {
    // We can type annotate variables.
    let logical: bool = true;

    let a_float: f64 = 1.25;     // Standard notation
    let an_integer = 5i32;       // Suffix notation

    // Or a default will be used.
    let default_float = 3.27;    // f64
    let default_integer = 7;     // i32

    // A type can also be inferred from context
    let mut inferred_type = 12      // Type  i64 is inferred from another line
    inferred_type = 482838293923i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12;       // Mutable i32
    mutable = 21;

    // Error! The type of a variable can't be changed.
    mutable = true;

    // Variables can be overwritten with shadowing (i.e. 'let')
    let mutable = true;
}
