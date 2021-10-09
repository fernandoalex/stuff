#[derive(Debug)]
enum UsState {
    //Alabama,
    Alaska,
}

enum Coin {
    //Penny,
    //Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        //Coin::Penny => 1,
        //Coin:: Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let cent = Coin::Dime;
    
    println!("value is {}", value_in_cents(cent));
    println!("value is {}", value_in_cents(Coin::Quarter(UsState::Alaska)));
}
