// Use GIT to see the history of this file


// https://doc.rust-lang.org/stable/book/ch10-02-traits.html
// fn largest<T: PartialOrd>(list: &[T]) -> T {
//
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest numern is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("the largest char is {}", result);
}
