pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: String) -> String {
    //return "Hello, Dave".to_string(); // fails test
    return format!("Hello, {}", name); // passes test
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 6,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 6,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn doesnt_add_three() {
        assert_ne!(5, add_two(2));
    }

    /*
    #[test]
    fn fails() {
        panic!("This test fails");
    }
    */

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol".to_string());
        assert!(result.contains("Carol"),
                "Greeting did not contain name; value was {}",
                result);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("I'm panicking right now!");
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn test_panic_w_expected_message() {
        if 0 < 100 {
            panic!("less than or equal to 100");
        }
    }

    // you can also specify return types for unit tests to ensure they return expected values
    /*
    * #[test]
    * fn it_works() -> Result<(), String> {
    *   if 2 + 2 == 4 {
    *       Ok(())
    *   } else {
    *       Err(String::from("two plus two does not equal four"))
    *   }
    * }
    * */

    // There are flags that cann be set to
    // You can set cargo test -- --test-threads to determine if you want tests to run consecutively, etc.
    // You can set cargo test -- --show-output to show the output of the tests
    // You can specify the name of specific tests to run only needed tests on command
    // You can specify a part of the test names to run multiple that match that pattern
    // You can flag with #[ignore] over specific tests unless they are specifically requested
    // You can set cargo test -- --ignored to run the ignored tests
    // You can set cargo test -- --include-ignored to run the normal and ignored tests together
}
