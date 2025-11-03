fn main() {
    let x = (let y = 6);
}

// Output:
/*
    error: expected expression, found `let` statement
    --> statement_error_case.rs:2:14
    |
    2 |     let x = (let y = 6);
    |              ^^^
    |
    = note: only supported directly in conditions of `if` and `while` expressions

    warning: unnecessary parentheses around assigned value
    --> statement_error_case.rs:2:13
    |
    2 |     let x = (let y = 6);
    |             ^         ^
    |
    = note: `#[warn(unused_parens)]` on by default
    help: remove these parentheses
    |
    2 -     let x = (let y = 6);
    2 +     let x = let y = 6;
    |

    error: aborting due to 1 previous error; 1 warning emitted
*/