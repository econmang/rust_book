/*
* Vector: Stores a variable number of values next to teach other
* String: A collection of characters
* Hash Map: Associates values with keys > particular implementation of the more general data
*           structure called map
* */

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // the vec! macro can be used to create a new vector
    let v_macro = vec![1, 2, 3];
    println!("The values in the vector created by the vec! macro:");
    for v_i in v_macro {
        println!("{}", v_i);
    }

    // you can initialize via the new() operator
    println!("The values in the vector created by the Vec::new() and .push() method:");
    let mut v: Vec<i32> = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    // This first loop does not consume the vector, v, but the second commented one will
    for v_i in &v {
        println!("{}", v_i);
    }
    /*
    for v_i in v {
        println!("{}", v_i);
    }
    */

    println!("Accessing new vec by using the &v_2[2] and v_2.get(2):");
    let v_2 = vec![1,2,3,4,5];
    let third: &i32 = &v_2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v_2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    println!("The first access method returns an &i32 and the second returns an Option<&i32>.");
    println!("Note: If you have a reference to an element, you cannot mutate a vector.");

    let mut update_vec = vec![100, 32, 57];
    println!("You can use a mut reference while iterating through a vector to update the value at the index.");
    println!("Original vec:");
    for i in &update_vec {
        println!("{}", i);
    }
    for i in &mut update_vec {
        *i += 50;
    }
    println!("Updated vec (each with 50 added to the value):");
    for i in &update_vec {
        println!("{}", i);
    }

    println!("\nEnums can be used to store multiple types of values in a vector.");
    println!("This example uses a SpeadsheetCell enum that has Int, Text, and Float variants.");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for val in &row {
        match val {
            SpreadsheetCell::Int(num) => println!("{}", num),
            SpreadsheetCell::Float(num) => println!("{}", num),
            SpreadsheetCell::Text(text) => println!("{}", text),
        }
    }
    println!("Note: Dropping a vector drops its elements. When it goes out of scope, it is freed from memory.");

    println!("s is a string created using String::new(). Then it is assigned to a new string value using the to_string() method.");
    println!("Note: Any type that implements the Display trait can utilize the to_string() method.");
    let mut s = String::new();
    println!("Originally s is the empty str: {}", s);
    let data = "initial contents";
    s = data.to_string();
    println!("s is then given the value: {}", s);
    println!("String can also be initilaized by the String::from() method passed a string literal.");
    let s = String::from("Initial contents.");
    println!("s is then reinitialized with the String::from() method, yielding: {}", s);
    println!("Note: Strings are UTF-8 encoded, so any properly encoded data can be sotred in them.");
    println!("\nWe can grow a string using the push_str() method:");
    let mut s1 = String::from("foo");
    println!("s1: {s1}");
    let s2 = "bar";
    println!("s2: {s2}");
    s1.push_str(s2);
    println!("s1 after push_str: {s1}");
    println!("We can also grow a string by pushing a character onto it with the push() method:");
    let mut s3 = String::from("lo");
    println!("s3 before 'l' is pushed: {s3}");
    s3.push('l');
    println!("s3 after 'l' is pushed: {s3}");
    println!("If you have two variables of the same type, you can also use the + operator to combine them:");
    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    println!("s4: {}, s5: {}", &s4, &s5);
    println!("s4 + s5 = {}", s4 + &s5);

    println!("\nThe format! operator can also aid in creating format strings to display string data:");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = format!("{s1}-{s2}-{s3}");
    println!("Formatted string that delimited three string values with hyphens: {}", s4);
    println!("Indexing into strings is not directly available. You can the byte as a u8 for the char and then convert it... but difficult.");


}
