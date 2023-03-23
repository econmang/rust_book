// Boxes enable recursive types: Types that have another vlaue the same type as part of itself.
// Likely going to try to use Boxes to implement linked lists. Obvi the std lib linked list is
// the one we'd use, but I want to try to implement one in Rust myself.

// A cons list is a data struct that comes from Lisp. Cons function in Lisp is short for
// "construct function". It's made up of nested pairs, and is the Lisp version of a linked
// list.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::rc::Rc;
use std::cell::RefCell;

enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

#[derive(Debug)]
enum RefCellList {
    RefCellCons(Rc<RefCell<i32>>, Rc<RefCellList>),
    RefCellNil,
}

/*
* Interior mutability is a design pattern in Rust that allows you to mutate data even when there
* are immutable references to that data. This can be done using the RefCell<T> smart pointer type.
* Note: Rc and RefCell can only be used in single-threaded instances
*
* 
* */

use crate::List::{Cons, Nil};
use crate::RcList::{RcCons, RcNil};
use crate::RefCellList::{RefCellCons, RefCellNil};

// Defining our own smart pointer: 
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
use std::mem::drop;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Deref coercion converts a reference to a type that implements the Deref trait into a reference
// of another type. Example &String to &str conversion because &String implements the Deref trait
// so that it returns &str.
fn hello(name: &str) {
    println!("Hello, {name}!");
}

// The Drop trait allows you to cusomize what happens when a value is about to go out of scope
// Smart pointers implement this trait by deallocating space on the heap associated with its value
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


fn main() {
    /*
    * Boxes are used when: 
    *
    * - You have a type whose size can't be known at compile time, and you want
    * to use a val in a context that requires an exact size.
    * - When you have a large amount of data and you want to transfer ownership but want to ensure
    * the data won't be copied when you do so.
    * - When you want to own a value and you care only that it's a type that implements a
    * particular trait rather than being of a specific type
    * */
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    match list {
        Cons(_,_) => println!("Linked list has defined head node."),
        Nil => println!("No head on linked list!!"),
    }

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    let y = Box::new(x);
    assert_eq!(5, *y);
    let y = MyBox::new(x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    // deref corecion in action
    hello(&m);
    // as opposed to having to write hello(&(*m)[..]);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // Although an available method can be called: c.drop();
    // You cannot call a deconstructor method directly because
    // the main thread calls this on its own.
    // Instead, we can use std::mem::drop func
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    /*
    * In the case of a graph data structure, multiple edges point to the same node, meaning you'd
    * need more than one reference to a value.
    * For this use case we'd use a reference counting smart pointer Rc<T>. It keeps a count of how
    * many references are made to it so you can deallocate it once it no longer has any references.
    * */
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    //Rc::clone increases the reference count for `a`
    let _b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = RcCons(4, Rc::clone(&a));
        println!("count afer c goes out of scope = {}", Rc::strong_count(&a));
    }
    println!("count afer c goes out of scope = {}", Rc::strong_count(&a));

    // This below would fail because x is not mutable but y is making a mut reference to it.
    //let x = 5;
    //let y = &mut x;
    // Having mutliple owners of mutable data by combining Rc<T> and RefCell<T>
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RefCellCons(Rc::clone(&value), Rc::new(RefCellNil)));

    let b = Rc::new(RefCellCons(Rc::new(RefCell::new(3)), Rc::clone(&a)));
    let c = Rc::new(RefCellCons(Rc::new(RefCell::new(4)), Rc::clone(&a)));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // Note: Even with Rust's memory safety guarantees, you can still leak memory.
    // By creating a refernce cycle, you can leak memory
    // To avoid cycles, you can utilize Weak<T> instead of Rc<T> using a Rc::downgrade call
}
