    /*
    * A trait defines functionality a particular type has and can share with other types. We can use
    * traits to define shared behavior in an abstract way. We can use trait bounds to specify a generic
    * type can be any type that has a certain behavior.
    *
    * A trait is similar to an interface in other languages, but there are differences
    */
    pub trait Summary {
        // all fn's from a trait must be implemented, so if you add this function below,
        // NewsArticle and Tweet would both have to utilize this function as well
        // fn summarize_author(&self) -> String;

        // by defining a method body, a trait has  a default implementation
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // traits as a parameter:
    // this is actually syntactic sugar for trait bound syntax:
    /*
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    */
    // The trait bound syntax:
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    /* Allows us to create fn's where multiple params use the bound trait:
    * pub fn notify<T: Summary>(item1: &T, item2: &T)...
    *
    * You can specify multiple bound traits using the + syntax:
    * pub fn notify<T: Summary + Display>(item1: &T, item2: &T)...
    *
    * You can also create a more than one bound trait:
    * fn some_Function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32...
    *
    * This can be expanded out to a where clause
    * fn some_function<T,U>(t: &T, u: &U) -> i32
    * where
    *   T: Display + Clone,
    *   U: Clone + Debug,
    *   {...}
    *
    * */

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
