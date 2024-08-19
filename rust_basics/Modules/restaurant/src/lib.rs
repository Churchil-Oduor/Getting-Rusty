mod front_of_house {
    pub mod hosting {
        pub fn add_to_waiting() {}       
        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {
            serve_order();
            crate::front_of_house::hosting::seat_at_table();
        }
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    let order2 = back_of_house::Appetizer::Soup;
    let order3 = back_of_house::Appetizer::Salad;

    meal.toast = String::from("Wheat");
    println!("I'd like to have {} instead", meal.toast);
}


fn deliver_order() {}
mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
        super::eat_at_restaurant();
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
}

