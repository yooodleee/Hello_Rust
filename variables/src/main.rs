fn main() {
    let mut x = 5;  // add mut keyword -> generate mutable variable x 
    println!("The value x is: {x}");
    x = 6;          // x = 5 -> x = 6
    println!("The value x is: {x}");
}

// output: 
/*
   Compiling variables v0.1.0 (C:\Users\dhals_zn0ga5j\python_seed\Hello_Rust\Hello_Rust\variables)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.85s
     Running `C:\Users\dhals_zn0ga5j\python_seed\Hello_Rust\Hello_Rust\variables\target\debug\variables.exe main.rs`
The value x is: 5
The value x is: 6
*/