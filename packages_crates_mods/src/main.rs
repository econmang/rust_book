/*
* Packages: A Cargo feature that lets you build, test, and share crates
* Crates: A tree of modules that produces a library or executable
* Modules and use: Let you control the organization, scope, and privacy of paths
* Paths: A way of naming an item, such as a struct, function, or module
*
* Crate is the smallest amount of code the Rust compiler consider at a time. Binary crates are
* programs you can compile to an executable.
*
* Library crates don't have a main function, and they don't compile to an executable. Instead they
* define functionality intended to be shared with multiple projects.
*
* The crate root is a source file that the Rust compiler starts from and makes up the root module
* of your crate.
*
* Package is a bundle of one or more crates that provide a set of functionality.
*
* Modules let us organize code within a crate for readability and reusability.
* They also control the privacy of items, with private as a default.
*
* */

// The use keyword can be used to pull crates into scope
use packages_crates_mods::front_of_house::serving;
// the as keyword can be used to rename a crate
use std::io::Result as IOResult;

// when a used crate is brought into scope, it is considered private to the current scope
// to utilize it in calling crates, use the `pub use` pattern
/*
* use crate::front_of_house::hosting; // private
* pub use crate::front_of_house::hosting; // public
* */

// Cargo.toml is used to bring in external crates
// the below use statement is utilizing the external rand crate
use rand::Rng;

// With the glob operator, we can bring all public items defined in a path into scope
use std::collections::*;

/// Three slashes are used to create documentation comments
///
/// This project is just used to house examples of creating mods and crates and pulling them into
/// scope.
///
/// You can open the documentation for a project using cargo doc --open

fn main() {
    serving::take_order();
    serving::take_payment();

    let _secret_number = rand::thread_rng().gen_range(1..=100);
    // pulled in by the collections glob
    let _h_map = HashMap::from([("a",2)]);
}


