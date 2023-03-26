## Unsafe Rust

Rust's "second lang" that doesn't enforce these memory safety guarantees.

This can allow you to:
- Deref a raw pointer
- Call an unsafe func or method
- Access or modify a mutable static var
- Implement an unsafe trait
- Access files of unions

You can create an unsafe block, unsafe func, and unsafe enum by using the `unsafe` keyword prepended to those definitions

## Advanced Traits

Associated types connect a type placeholder with a trait such that the trait defn's can use these placeholder types in their signatures.

```
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// vs.

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {}
}
```
