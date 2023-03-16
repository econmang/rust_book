#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip --
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

/*
* In place of a null value, Rust offers the Option enum. 
* where it takes a generic and can return Some(T) or None.
* */
fn plus_one(x: Option<i32>) -> Option<i32> {
    // matches are exhaustive
    // so you must create an arm for each potential mathc
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }

    /*
    // To limit the amount of typing you can create a catch-all pattern
    match dice_roll {
        3 => drink_health_potion(),
        9 => attack(),
        other => move_player(other),
    }

    // If you don't need to use the argument on the match arm, you can just use the placeholder (_)
    match dice_roll {
        3 => drink_health_potion(),
        9 => move_player(9),
        _ => reroll(),
    }
    */
}

fn main() {
    let q_alaska_coin = Coin::Quarter(UsState::Alaska);
    println!("{}",value_in_cents(q_alaska_coin));

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // Concise control flow with if let
    // In the case where you are only matching for a Some() value:
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // You can make it more concise with the following:
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The max found through the if-let syntax is configured to be {}", max);
    }
}
