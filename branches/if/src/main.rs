fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let num = 3;

    if num {
        println!("num was three");
    }
}

// Output: 
/*
error[E0308]: mismatched types
  --> main.rs:12:8
   |
12 |     if num {
   |        ^^^ expected `bool`, found integer

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.

*/