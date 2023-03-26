// used by the @ Bindings example
enum Message {
    Hello { id: i32 },
}

fn main() {
    // What happens when you use a named variable that exists for a match arm
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("Default case: x = {:?}, y = {y}", x);

    // @ Bindings
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7,
            } => println!("Found an id range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id }=> println!("Found some other id: {id}"),
    }
}
