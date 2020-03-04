mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod back_of_house {
    
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    //
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about the bread
    //
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
