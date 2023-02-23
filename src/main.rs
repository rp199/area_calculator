fn main() {
    let rectangle1 = Rectangle {
        width: 10,
        height: 20,
    };

    let square1 = Rectangle::square(9);

    dbg!(&rectangle1);
    dbg!(&square1);

    println!("rectangle1 area: {}", rectangle1.area());
    println!("square1 area: {}", square1.area());

    println!("Can rectangle1 hold square1? {}", rectangle1.can_hold(&square1));
    println!("Can square1 hold rectangle1? {}", square1.can_hold(&rectangle1));
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
