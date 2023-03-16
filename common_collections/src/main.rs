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
}
