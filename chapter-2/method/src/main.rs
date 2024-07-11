struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}
impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }

}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels", rect1.area());
    if rect1.width() {
        println!("The rectangle has a nonzero width, it is {}", rect1.width);
    }
}
