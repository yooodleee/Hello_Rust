fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}

// Output: 
/*
error[E0425]: cannot find value `s` in this scope
 --> doublefree.rs:5:28
  |
5 |     println!("{}, world!", s);
  |                            ^ help: a local variable with a similar name exists: `s1`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
*/