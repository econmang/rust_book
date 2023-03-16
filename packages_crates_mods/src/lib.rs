// The pub keyword is used to make a mod/struct/function public
pub mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {
            println!("Order taken");
            serve_order();
        }

        fn serve_order() {
            println!("Order served (private function called internally to mod)");
        }

        pub fn take_payment() {
            println!("Meal done, payment taken");
        }
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super keyword can be used to pull from a parent mod
        // instead of utilizing the absolute path to the crate
        super::deliver_order();
    }

    fn cook_order() {}
}
