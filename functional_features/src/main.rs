#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        /*
        * Closures are anonymous functions you can save in a variable or pass as arguments to other
        * functions.
        *
        * The below line uses a closure on the unwrap_or_else() argument
        *
        * */
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            };
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

    // Example closures:
    fn _add_one_v1 (x: u32) -> u32 { x + 1 }
    let _add_one_v2 = |x: u32| -> u32 { x + 1 };

    let example_closure = |x| x;
    // the first line is used to help infer the type of the closure
    let _s = example_closure(String::from("hello"));
    // because the first line causes the closure the infer type String, the below will cause an
    // mismatch type error:
    //let n = example_closure(5);

    /*
    * The iterator pattern allows you to perform some task on a sequence of items in turn
    * the syntax for this in Rust is for val in iterable {  }
    * */
}
