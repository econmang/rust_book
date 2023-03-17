use generics_traits_lifetimes::aggregator::{Tweet, NewsArticle, Summary, notify};

/*
* Generics: Abstract stand-ins for concrete types or other properties
* Traits: Can be used to define behaivor; when combined with generics, does so in an abstract way
* Lifetimes: A variety of generics that give the compiler information about how references relate
*            to one another
*
* Monomorphization: The process of turning generic code into specific code by filling in concrete
* types at compile time. Ensure that generics do not slow down your code.
*/

/*
// Rather than repeating this function for char, f64, etc. we can utilize generics
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}
*/

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

// struct defn using Generics
struct Point<T, U> {
    x: T,
    y: U,
}
// Generics in function defns
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        return &self.x;
    }
}

/* Result is an example of an enum defn using Generics
enum Result <T, E> {
    Ok(T),
    Err(E),
}
*/

fn main() {
    /*
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
    */

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['a', 'c', 't', 't', 'q', 'f', 'e'];
    let result = largest(&char_list);
    println!("The greatest-valued char is {}", result);

    let example_pt = Point { x: 5, y: 4.0 };
    println!("Example Point values: x: {}, y: {}", example_pt.x(), example_pt.y);

    let tweet = Tweet {
        username: String::from("test_user"),
        content: String::from("Potatoes. That's it. That's the tweet"),
        reply: false,
        retweet: false
    };
    let news_article = NewsArticle {
        headline: "Extra, Extra, Read All About It!".to_string(),
        location: "Bakersfield, CA".to_string(),
        author: "Buck Owens".to_string(),
        content: "Bakersfield Condors fight at another hockey game.".to_string()
    };
    println!("New Tweet: {}", tweet.summarize());
    println!("New Article: {}", news_article.summarize());
    notify(&news_article);

    println!("\nNote: The main aim of lifetimes is to prevent dangling references.");
    println!("The borrow checker compares scope to determine whether all borrows/references are valid.");
    /*
    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    */
    // The above fails because the signature declare it doesn't know which lifetime (x or y) it
    // will borrow it's value from. So it needs a generic lifetime parameter to define the
    // relationship between the references so the borrow checker can perform its analysis.
    /*
    * &i32 // reference
    * &'a i32 // a reference with an explicit lifetime
    * &'a mut i32  // a mutable reference with an explicit lifetime
    * */
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            return x;
        } else {
            return y;
        }
    }
    // the above declares that all params must have the same lifetime 'a
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // The below would error because the borrowed value for the function call frees after the above
    // liftime:
    // println!("The longest string is {}", result);
}
