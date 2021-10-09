
fn main() {
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("the third value is: {}", third);

    // this returns us a `Option<&T>`
    match v.get(2) {
        Some(third) => println!("the third element is {}", third),
        None => println!("There is no third element"),
    }

    //let does_not_exist = &v[100]; // this will make the program crash
    //let does_not_exist = v.get(100); // this will return a None type and will not panick
    
    let v1 = vec![100, 32, 57];

    for i in &v1 {
        println!("value is {}", i);
    }

    let mut v2 = vec![100, 32, 66];

    for i in &mut v2 {
        *i += 50;
    }

    for i in &v2 {
        println!("new value is {}", i);
    }


// this is valid code
    //enum SpreadsheetCell {
        //Int(i32),
        //Float(f64),
        //Text(String),
    //}

    //let row = vec![
        //SpreadsheetCell::Int(3),
        //SpreadsheetCell::Text(String::from("blue")),
        //SpreadsheetCell::Float(10.12),
    //];
}
