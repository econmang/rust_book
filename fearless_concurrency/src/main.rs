//use std::rc::Rc;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
            thread::sleep(Duration::from_millis(200));
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
            thread::sleep(Duration::from_millis(200));
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
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Received from spawned threads: {received}");
    }
    // END

    // START
    println!("\nMutex example; updating a value:");
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("Mutex m value after update: {:?}", m);
    // END

    // START
    println!("\nSharing a mutex value between threads:");
    // Rc<T> is limited to only single threaded situations
    // Arc<T> is a tyle similar that can be used safely in concurrenct situations.
    // Note; A on Arc stands for atomic, so it's an atomically reference counted type
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result after spawning 10 threads to handle the mutex value: {}", *counter.lock().unwrap());
    // END
}
