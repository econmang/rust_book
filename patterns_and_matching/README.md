## Patterns and Matching

Patterns: Special syntax in Rust for matching against the structure of types.
Consists of combination of the following:
    - Literals
    - Destructed arrays, enums, structs, or tuples
    - Variables
    - Wildcards
    - Placeholders

### `match` Arms
Match expressions are defined using the keyword `match`, a value to match on, and one or more match arms.

### `if let` Arms
If-let expressions are defined using the keywords `if let`:

```
fn main() {
    let favorite_color: Option<&str> = None;
    if let Some(color) = favorite_color {
        println!("Your favorite color is {color}.");
    } else {
        println!("You don't have a favorite color");
    }
}
```

### `while let` Arms
While-let expressions are defined using the keywords `while let`:

```
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### `for` Arms
For loops are defined using the keywords utilizing a for y in x pattern that leverages iterators:

```
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{value} is at {index}");
}
```

### `let` Statements
Let statements allow you to set a variable (or variables) using a binding pattern:

```
let x = 5;
let (x, y, z) = (1, 2, 3);
```

### Functional parameters
In addition to being able to specify an individual variable param for a function defn, you can also deconstruct params using a pattern:

```
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

### Pattern types
Irrefutable patterns will match for any possible value passed (example: let x = 5;).
Refutable patterns can fail to match for some possible value (example: if let Some(x) = a_value).

### Matching literals
```
let x = 1;

match x {
    1 => println!("one");
    2 => println!("two");
    3 => println!("three");
    _ => println!("anything");
}
```

### Matching named variables

```
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50");
    Some(y) => println!("Matched, y = {y}");
    _ => println!("Default case, x = {:?}", x);
}

println!("at the end: x = {:?}, y = {y}", x);
```
The named variable within the above match pattern will carry y through to the end of the match pattern.
The final println!() call will utilize the value of y in the main scope, so it will show y = 10.

### Matching ranges of values with ..=

```
let x = 5;

match x = {
    1..=5 => println!("one through five!"),
    _ => println!("something else"),
}

let x = 'c';

match c = {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

### Destructuring struct fields
You can use the patterns deconstructs fields from structs:
```
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7};

    // Shorthand deconstruction
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // Deconstruct using match arms
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis at: ({x}, {y})"),
    }
}
```

## Desconstructing enums

You can utilize match arms to deconstruct enum values
```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message:ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, blue {b}");
        }
    }
}
```

Similar patterns can be utilized to deconstruct nested structs and enums

You can ignore a value using the _ character in a pattern
If you have multiple values you'd like to ignore, you can specify the parts needed and then use .. to ignore remaining values

### @ Bindings

The at operator (@) lets us create a variable that holds a value at the same time as we're testing for a pattern match.
See main.rs for code in practice.
