fn main() {
    plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;      /* Correct Case: x + 1 (expression) */
}

// Output: 
/*
    error[E0425]: cannot find value `x` in this scope
    --> mismatched_types.rs:4:35
    |
    4 |     println!("The value of x is: {x}");
    |                                   ^ not found in this scope

    error[E0308]: mismatched types
    --> mismatched_types.rs:7:24
    |
    7 | fn plus_one(x: i32) -> i32 {
    |    --------            ^^^ expected `i32`, found `()`
    |    |
    |    implicitly returns `()` as its body has no tail or `return` expression
    8 |     x + 1;      /* Correct Case: x + 1 (expression) */
    |          - help: remove this semicolon to return this value

    error: aborting due to 2 previous errors

    Some errors have detailed explanations: E0308, E0425.
    For more information about an error, try `rustc --explain E0308`.
*/