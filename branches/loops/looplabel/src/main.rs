fn main() {
    let mut count = 0;
    'counting_up: loop {            // 'counting_up: loop label
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }   

        count += 1;
    }
    println!("End count = {count}");
}

// Output:
/*
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
*/