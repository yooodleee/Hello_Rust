fn main() {
    let x = 5;

    // difference between mut and let?
    let spaces = "    ";        // string type
    let spaces = spaces.len();  // reusable name(spaces) -> good case

    /*
    bad case -> use mut keyword
    let mut spaces = "    ";    // expected due to this value("    ")
    spaces = spaces.len();      // expected `&str`, found `usize`(error)
     */

    let x = x + 1; // shadowed

    {
        let x = x * 2; // shadowed
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// output: 
/*
   Compiling shadowed v0.1.0 (C:\Users\dhals_zn0ga5j\python_seed\Hello_Rust\Hello_Rust\shadowed)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.79s
     Running `C:\Users\dhals_zn0ga5j\python_seed\Hello_Rust\Hello_Rust\shadowed\target\debug\shadowed.exe main.rs`
The value of x in the inner scope is: 12
The value of x is: 6
*/