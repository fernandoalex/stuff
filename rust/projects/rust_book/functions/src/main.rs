fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let x = a_function_that_return();

    println!("the value of the new variable is: {}", x);
}

fn another_function(x: i32, y:i32) {
    println!("the value of x is: {}", x);
    println!("the value of y is: {}", y);
}

fn a_function_that_return() -> i32 {
    5
}
