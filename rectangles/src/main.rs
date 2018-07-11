#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    let rect1 = Rectangle { width: 10, height: 40 };
    let rect2 = Rectangle::square(50);

    println!("rect is {:#?}", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("can rect hold rect1? {}", rect.can_hold(&rect1));
    println!("can rect1 hold rect? {}", rect1.can_hold(&rect));
    println!("can rect2 hold rect1? {}", rect2.can_hold(&rect1));
}
