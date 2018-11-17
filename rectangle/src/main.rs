#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width >= other.width && self.height >= other.height) ||
            (self.width >= other.height && self.height >= other.width)
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 40 };
    let rect2 = Rectangle { width: 35, height: 23};

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
    println!("Rect1: {:#?}", rect1);
    println!("Rect2: {:#?}", rect2);
}
