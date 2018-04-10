#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 50,
        height: 50,
    };

    let small_rectangle = Rectangle {
        width: 40,
        height: 40,
    };

    let big_rectangle = Rectangle {
        width: 60,
        height: 60,
    };

    let square = Rectangle::square(30);

    println!("{:#?}", rectangle);
    println!("Rectangle area: {}", rectangle.area());

    println!(
        "Can rectangle hold small rectangle? {}",
        rectangle.can_hold(&small_rectangle)
    );

    println!(
        "Can rectangle hold big rectangle? {}",
        rectangle.can_hold(&big_rectangle)
    );

    println!("Square: {:#?}", square);
}
