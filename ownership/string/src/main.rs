fn main() {
    let mut s = String::from("hello");      /* ::(namespace operator) */

    s.push_str(", world!");     /* push a literal to "hello" */

    println!("{}", s);      /* `hello, world!` */
}

// Output:
/*
hello, world!
*/