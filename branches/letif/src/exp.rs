fn main() {
    let condition = true;

    let number = if condition {5} else {"six"};

    println!("The value of number is: {number}");
}

// Output:
/*
error[E0308]: `if` and `else` have incompatible types
 --> exp.rs:4:41
  |
4 |     let number = if condition {5} else {"six"};
  |                                -        ^^^^^ expected integer, found `&str`
  |                                |
  |                                expected because of this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
*/