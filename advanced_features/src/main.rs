unsafe fn dangerous() {}

fn main() {
    println!("Hello, world!");
    unsafe {
        dangerous();
    }

}
