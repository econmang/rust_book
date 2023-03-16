// Errors are broken into two main categories: Recoverable and Unrecoverable
// Recoverable errors can be handled. Unrecoverable errors are always symptoms of bugs.


// Most langs distinguish between errors with the same mechanism: exceptions.
// Rust uses Result<T, E> for recoverable errors and the panic! macro stops execution
// if it encounters and unrecoverable error

// To get a backtrace on a panic!, you can set RUST_BACKTRACE=1 when running the program

use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// You can propogate errors by returning them in the Result of a func
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// We can shortcut the error propogation process using the ? operator
fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    return Ok(username);
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    return fs::read_to_string("hello.txt");
}

// You can also shortcut Option<> types as well
fn last_char_of_first_line(text: &str) -> Option<char> {
    return text.lines().next()?.chars().last();
}

// Note: main can also be changed to return a result
fn main() {
    let hello_file_result = File::open("hello.txt"); // returns type of Result<T, E> where T is a generic

    /*
    // This will panic! with the provided expect message if hello.txt does not exist 
    let _hello_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project.");
    */

    let _hello_file = match hello_file_result {
        Ok(file) => { println!("File hello.txt exists in project"); file},
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {println!("hello.txt does not exist; creating it."); fc},
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    // you can also use the unwrap_or_panic() call to handle an error on an attempt to unwrap a val

    // Unwrap will unwrap a value if it exists, or panic! otherwise
    let _greeting_file = File::open("hello.txt").unwrap();

    let fn_result = read_username_from_file();
    let fn_short_result = read_username_from_file_shortcut();
    let fn_shorter_result = read_username_from_file_shorter();
    let fn_shortest_result = read_username_from_file_shortest();
    match fn_result {
        Ok(_) => println!("OK"),
        Err(e) => println!("{:?}",e),
    }
    match fn_short_result {
        Ok(_) => println!("OK"),
        Err(e) => println!("{:?}",e),
    }
    match fn_shorter_result {
        Ok(_) => println!("OK"),
        Err(e) => println!("{:?}",e),
    }
    match fn_shortest_result {
        Ok(_) => println!("OK"),
        Err(e) => println!("{:?}",e),
    }

    let test_str = "this is a test string";
    println!("{}",&test_str.to_string());
    let last_char = last_char_of_first_line(&test_str);
    match last_char {
        Some(c) => println!("Final char: {c}"),
        _ => println!("No final character find"),
    }
}
