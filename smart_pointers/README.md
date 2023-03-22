### Notes:

Pointers are a general concept for a variable that contains an address in memory. This address refers to ("points at") some other data.
Rust references (variables prepended with &) are the most common pointer in Rust.

Smart pointers: data structs that act like pointer but have additional metadata and capabilities.
While references only borrow to data, smart pointers oflten own the data they point to.

Smart pointers implement the Deref and Drop traits

Common std lib smart pointers:
    - Box<T>: Allocates values on the heap
    - Rc<T>: A reference counting type that enables multiple ownership
    - Ref<T> and RefMut<T>, accessed through RefCell<T>: a type that enforces borrowing rules at runtime instead of compile time

Interior Immutability: Pattern where an immutable type exposes an API for mutating an interior value.
