## What We're Covering

Concurrent Programming: Where different parts of a program execute independently.
Parallel Programming: Where different parts of a program execute at the same time.

We will cover:
- How to create threads to run multiple pieces of code at the same time
- Message-parsing Concurrency: Where channels send messages between threads
- Shared-state Concurrency: Where multiple threads have access to some piece of data
- The `Sync` and `Send` traits, which extend Rust's concurrency guarantees to user-defined types as well as types provided by the std lib.

## Using Threads to Run Code Simultaneously

Process: An executed program's code running in the OS.
Threads: OS spawned executive processes that can be used to run independent parts of the overall process.

Running multiple threads can lead to: Race Conditions, Deadlocks, and other bugs.
Message passing: Where threads or actors communicate by sending each other messages containing data.
                 A channel is closed if either the transmitter or receiver half is dropped.

## Shared-State Concurrency

Threads can also forgo messages by accessing the same shared data, though sometimes shared-state is push back against by those who believe in the usage of message passing.

Mutex (Mutual Exclusion): Allows only one thread to access some data at any given time. To access data in a mutex, a thread must signal by acquiring a lock on the mutex. Two rules that lead to difficulty working with mutexes:

- You must attempt to acquire the lock before using the data.
- When you're done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

## Extensible Concurrency w/ Sync and Send Traits

The std::marker traits Sync and Send need to be implemented to write concurrency features.
Send: Used to allow transference of Ownership between threads.
Sync: Can be used to indicate a type is safe to be referenced by multiple threads.


