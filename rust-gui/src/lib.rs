pub trait Drawable {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<Drawable>>,
}

impl Screen {
    pub fn render(&self) {
        self.components
            .iter()
            .for_each(|component| component.draw());
    }
}

pub struct Button {
    pub height: u32,
    pub width: u32,
    pub label: String,
}

impl Drawable for Button {
    fn draw(&self) {
        println!(
            "Drawing button `{}` with width: {}, height: {}",
            self.label, self.width, self.height
        );
    }
}
