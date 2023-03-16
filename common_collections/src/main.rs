/*
* Vector: Stores a variable number of values next to teach other
* String: A collection of characters
* Hash Map: Associates values with keys > particular implementation of the more general data
*           structure called map
* */

use std::collections::HashMap;

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

    let s1 = String::from("hello");
   /* let h = s1[0]; // This will throw an error because you cannot directly index a String
    * Internally, a String is a wrapper over a Vec<u8> based on the UTF-8 encoding of the String
    * The byte length of a String w/ value Hola is 4
    * But the byte length of a String w/ value Здравствуйте is 24 because it takes that many bytes
    * to encode the Cyrillic letters.
    *
    * Strings can be viewed from the perspective of bytes, scalar values, and grapheme clusters
    * (the closest thing to what we would call letters).
    *
    */

    // Slicing Strings
    println!("Original String: {}", &s1);
    println!("Slice from 0 - 2 (non-inclusive): {}", &s1[0..2]);

    // Methods for iterating over strings:
    let string_to_iterate = String::from("Здравствуйте");
    println!("\nFull String to iterate through: {}", &string_to_iterate);
    println!("Iterating utilizing the .chars() method:");
    for c in string_to_iterate.chars() {
        println!("{c}");
    }
    println!("\nItereating utilizing the .bytes() method:");
    for b in string_to_iterate.bytes() {
        println!("{b}");
    }

    println!("\nHash Maps:");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("State of HashMap created with the ::new() method and having two key/value pairs inserted into it:");
    for score in &scores {
        println!("{}: {}", score.0, score.1);
    }
    println!("Note: We pulled an individual name requiring us to iterate and then pull score.0 and score.1 to get the key/value pair.");
    println!("\nWe can also iterate using an aliased tuple like (key, value) to pull from the HashMap:");
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    println!("\nGrabbing a value from the hashmap using the get() method:");
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Key passed: {team_name}; Score retrieved: {score}");
    println!("Note: Values that implement the Copy trait, like i32, will be copied in tot he HashMap upon insert. Owned values (like Strings) will be moved and the HashMap will become the owner.");

    println!("\nTo update a HashMap, you can just insert the same key and it will overwrite the previous value:");
    println!("Original Scores: {:?}", scores);
    scores.insert(String::from("Blue"), 25);
    println!("Scores after inserting a new val for \"Blue\": {:?}", scores);
    println!("\nYou can also update value based on the old value:");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count =  map.entry(word).or_insert(0);
        *count += 1;
    }
    // split_whitespace() returns slices of the string based on where the whitespace is.
    // or_insert returns a mutable reference (&mut V) to the value of the specified key
    // We dereference the count using the (*) asterisk to change the direct value
    // The mut ref goes out of scope after the end of the loop, making it safe/allowed borrowing
    println!("Map after looping through words and counting the instances of each: {:?}", map);
}
