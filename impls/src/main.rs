trait Shape {
    fn area(&self) -> f64;

}
struct Circle {
    radius: f64,

}
 impl Circle {
    fn new(radius: f64) -> Self {
        Circle {radius}
    }
 }

  impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
  }
   fn main() {
    let circle = Circle::new (5.0);

    println!("The area of the circle is {:.2}", circle.area());
   }
