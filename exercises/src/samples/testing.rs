#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

struct Guess {
    value: u32,
}

impl Guess {
    fn new(value: u32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

fn add_two(num: i32) -> i32 {
    num + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let small_rectangle = Rectangle {
            length: 5,
            width: 1,
        };
        let big_rectangle = Rectangle {
            length: 8,
            width: 7,
        };

        assert!(big_rectangle.can_hold(&small_rectangle));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let small_rectangle = Rectangle {
            length: 5,
            width: 1,
        };
        let big_rectangle = Rectangle {
            length: 8,
            width: 7,
        };

        assert!(!small_rectangle.can_hold(&big_rectangle));
    }

    #[test]
    fn adds_two_on_positive_number() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn adds_two_on_negative_number() {
        assert_eq!(-2, add_two(-4));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
