fn main() {
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let new_number = 6;

    if new_number % 4 == 0 {
        println!("new_number is divisible by 4"); 
    } else if new_number % 3 == 0 {
        println!("new_number is divisible by 3");
    } else if new_number % 2 == 0 {
        println!("new_number is divisible by 2");
    } else {
        println!("numer is not divisible by 4, 3 or 2");
    }


    let another_condition = true;
    let another_number = if another_condition {
        5
    } else {
        6
    };
    println!("the value if number is: {}", another_number);
}
