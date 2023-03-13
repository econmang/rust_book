use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn print_delimeter(example_type: String) {
    let delim: String = String::from("--------------------------------------------------------------------");
    println!("\n{delim}\n{example_type}\n{delim}")
}

fn shadowing_example() {
    print_delimeter(String::from("Shadowing"));
    let spaces = "   ";
    println!("'spaces' originally referred to the string: '{spaces}'");
    let spaces = spaces.len();
    println!("using shadowing, it now refers to the length of the original string: {spaces}");
    println!("Note: You need to use the 'let' keyword when shadowing, otherwise you'll get a type error.");
}

fn expression_example() {
    print_delimeter(String::from("Expressions"));
    // Expressions include a function call, calling a macro, or utilizing curly brackets to create
    // a new scope block
    let y = {
        let x = 3;
        x + 1
    };
    println!("y received the following from the scope block: {y}");
    println!("Note: Expression statements like the one above do not end with a semi-colon.");
    println!("If a semi-colon is added, an expression becomes a statement, and does not return a value from the expression.");
}

fn loop_return_example() -> i32 {
    print_delimeter(String::from("Loop Returns"));
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // the statement after the 'break' will evaluate and return from this loop to result
            break counter * 2;
        }
    };
    println!("Counter is currently {counter}, causing result to be {result}");
    return result;
}

fn loop_label_example() {
    print_delimeter(String::from("Loop Labels"));
    let mut count = 0;
    println!("Loop label examples: We have an outer loop labelled 'counting_up, that will break when count == 2");
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining == {remaining}");
            if remaining == 9 {
                println!("Broke from inner loop");
                break;
            }
            if count == 2 {
                println!("Broke from 'counting_up (outer loop)");
                break 'counting_up;
            }
            remaining -= 1;
        }
        count +=1;
    }
    println!("End count = {count}");
}

fn loop_types_example() {
    print_delimeter(String::from("Loop Types"));
    println!("Loop Examples:");
    let mut number = 3;

    println!("While number isn't zero:");
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    println!("\nFor loops (Note: these process over iterator objects):");
    for i in (1..4).rev() {
        println!("{i}");
    }
    println!("LIFTOFF!!!");
}

fn string_memory_example() {
    print_delimeter(String::from("String Representation in Memory"));
    println!("Memory example to differentiate the stack vs the heap.");
    let first_prompt: &str = "This is an immutable &str so it is placed on the stack.";
    let second_prompt: &str = "This is another string literal. What follows is a mutable string reference that would be allocated to the heap:";
    let mut hello_str: String = String::from("Hello");

    hello_str += " world";
    println!("{first_prompt}\n{second_prompt}");
    println!("{hello_str}");

    println!("\nNow an example in borrowing str memory:");
    let s1: String = String::from("borrowed_str");
    let s2: String = s1.clone();
    println!("In order to okay the use of the same string object, we have to use the .clone() method on s1 when assigning s2.");
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;
    println!("Integer sharing is different (since the value is allocated on the stack instead of the heap, so no borrowing is necessary.");
    println!("x = {}, y = {}", x, y);
}

// Borrowing a string value
fn calc_string_len(s: &String) -> usize {
    return s.len();
}

fn print_str_and_len_example() {
    print_delimeter(String::from("Testing References w/o Ownership"));
    let s1 = String::from("hello");
    let in_func_len = s1.len();
    // ampersands allow you to reference a value without taking ownership of it
    let ex_func_len = calc_string_len(&s1);
    println!("The length of '{s1}' is {in_func_len}, as found by String's len() method.");
    println!("The length of '{s1}' is {ex_func_len}, as found by the externally borrowed string value in calc_string_len().");
}

/*
// This function will error when called since it tries to
// change a value through an immutable reference
fn change_str_blocked(some_string: &String) {
    some_string.push_str(", world");
}
*/

fn change_str(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn mutable_reference_example() {
    print_delimeter(String::from("Mutables References"));
    let mut s = String::from("hello");
    println!("Test to see what happens when passing a mutable reference to a String");
    println!("'s' before change_str(): {s}");
    change_str(&mut s);
    println!("'s' after change_str(): {s}");
    println!("Note: You need a mutable reference to be able to change value, and you can only have one mutable reference at a time.");
}

fn get_string_from_method_example() {
    print_delimeter(String::from("Evaluating Dangling References"));
    println!("Exploring how now to handle dangling references, and printing the result:");
    //let s = dangle();
    let s = no_dangle();
    println!("{s}");
}
fn no_dangle() -> String {
    let s = String::from("Hello");
    return s;
}
/*
fn dangle() -> &String {
    let s = String::from("hello");
    // If you return &String, it will fault because it returns a borrowed value with
    // nothing to borrow from (because the value gets destroyed after dangle runs.
    return &s;
}
*/

fn slice_exmples() {
    print_delimeter(String::from("Slices"));
    let orig_nums = [2, 8, 12, 42, 77, 3, 9, 42];
    let nums_slice: &[i32] = &orig_nums[3..5];
    let orig_str: String = String::from("THIS IS A TEST STRING");
    let str_slice: &str = &orig_str[10..14];

    println!("Original i32 array: {:?}", orig_nums);
    println!("Slice of i32 array: {:?}", nums_slice);
    println!("Original string: {orig_str}");
    println!("Slice of string: {str_slice}");
    println!("Note: Slices are references to parts of the memory occupied by the original object.");
    println!("For that reason, you cannot deallocate the original object if you still intend to use the slice.");
}

fn main() {
    const TURN_LIMIT: u32 = 10;
    let mut turn: u32 = 1;
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is: {secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        //let apples = 5; //immutable
        //let mut bananas = 5; //mutable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        /*
        * How to use placeholders in print statements
        let x = 5;
        let y = 10;
        println!("x = {x} and y + 2 = {}", y + 2);
        */

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                if turn < TURN_LIMIT {
                    println!("You win!");
                } else {
                    println!("You guessed the number, but took too many turns :(");
                }
                break;
            }
        }
        turn += 1;
    }
    shadowing_example();
    expression_example();
    loop_return_example();
    loop_label_example();
    loop_types_example();
    string_memory_example();
    print_str_and_len_example();
    mutable_reference_example();
    get_string_from_method_example();
    slice_exmples();
}
