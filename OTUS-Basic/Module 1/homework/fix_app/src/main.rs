fn main() {
    println!("Hello Rust!");
    let x = 10;
    if x > 5 {
        println!("x is greater than 5");
    } else {
        println!("x is 5 or less");
    }

    unused_function();
}

fn unused_function() {
    println!("This function is never called");
}
