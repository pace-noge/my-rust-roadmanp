
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
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle::square(20);

    println!("Rect1 area is {} square pixels.", rect1.area());
    println!("rect2 area is {} square pixels.", rect2.area());

    println!("rect1 can hold rect2? {}", rect1.can_hold(rect2));
    println!("rect1 width {} is greater than 0? {}", rect1.width, rect1.width());
}
