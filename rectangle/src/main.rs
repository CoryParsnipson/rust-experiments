#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let r = Rectangle {
        width: 34.0,
        height: 24.5,
    };

    let r2 = Rectangle {
        width: 12.4,
        height: 19.27,
    };

    println!("Rectangle is {:?}", r);
    println!("The area of this rectangle is {}.", r.area());

    println!("Can r hold r2? {}", r.can_hold(&r2));
}
