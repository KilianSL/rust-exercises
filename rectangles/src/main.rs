struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 5,
        height: 10,
    };

    let square = Rectangle::square(3);

    println!("The area is {}", rect.area());

    println!("Rect1 can contain square: {}", rect.can_contain(&square));
}
