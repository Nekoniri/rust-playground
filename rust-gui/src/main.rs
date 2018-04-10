extern crate rust_gui;

use rust_gui::{Button, Drawable, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Drawable for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing Selectbox with width: {}, height: {} and options: {:?}",
            self.width, self.height, self.options
        );
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                label: String::from("Button 1"),
                width: 50,
                height: 50,
            }),
            Box::new(Button {
                label: String::from("Button 2"),
                width: 60,
                height: 60,
            }),
            Box::new(SelectBox {
                width: 70,
                height: 70,
                options: vec![String::from("hi")],
            }),
        ],
    };

    screen.render();
}
