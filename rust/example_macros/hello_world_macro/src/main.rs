use make_hello::make_hello;

#[make_hello()]
fn something() {
    println!("Hello, something!");
}

fn main() {
    println!("Hello, world!");
}
