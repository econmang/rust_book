use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    /*
    // START
    // The spawned thread will be forced to stop execution early when the main thread hits its end
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // END
    */

    /*
    // START
    // thread::spawn returns a JoinHandle.
    // The JoinHandle is an owned value that, when we call join(),
    // will wait for its thread to finish
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    // END
    */

    // The placement of the join handler matters:
    // START
    println!("\nA spawned thread and main thread utilizing iterators:");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Now the spawned thread will process to completion before moving on
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // END

    // START
    // the move keyword within closure passed to thread::spawn because the closure will
    // take the ownership of the values it uses from the environment, thus transferring ownership
    // of those values from one thread to another.
    println!("\nVector from main thread is shared to a spawned thread using the move keyword within a closure:");
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // Because Rust cannot infer how long the spawned thread will run,
    // the below drop will cause an error
    //drop(v);

    handle.join().unwrap();
    // END

    // START
    // Rust's message sending implementation is called channels
    println!("\nMessage passing example:");
    // mpsc: mutliple producer, single consumer
    // tx, rx are traditional names assigned meaning trasmitter and receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // you can't use val after it is sent out of the thread:
        // println!("val is {}, val"); //This line would fail
    });

    let received = rx.recv().unwrap();
    println!("Message received from spawned thread: {received}");
    // END

    // START
    // Rust's message sending implementation is called channels
    println!("\nPassing multiple messages and pausing between each passed:");
    // mpsc: mutliple producer, single consumer
    // tx, rx are traditional names assigned meaning trasmitter and receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // you can't use val after it is sent out of the thread:
        // println!("val is {}, val"); //This line would fail
    });

    for received in rx {
        println!("Received form spawned thread: {received}");
    }
    // END

    // START
    // Rust's message sending implementation is called channels
    println!("\nMultiple producers passing messages to a single consumer in the main thread:");
    // mpsc: mutliple producer, single consumer
    // tx, rx are traditional names assigned meaning trasmitter and receiver
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Received from spawned threads: {received}");
    }

    // END
}
