mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
use crate::back_of_house::Breakfast as Bf;
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    let meal2 = Bf::summer("Hi");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    let order1 = back_of_house::Food::Soup(String::from("Okro"));
    let order2 = back_of_house::Food::Salad(String::from("Danish"));
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        crate::eat_at_restaurant();
    }

    fn cook_order() {}

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
    pub enum Food {
        Soup(String),
        Salad(String),
    }
}
