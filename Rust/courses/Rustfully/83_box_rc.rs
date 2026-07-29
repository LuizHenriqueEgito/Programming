trait Shape {
    fn area(&self) -> f64
}

struct Circle {
    radius: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Square {
    side: f64
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.ride * self.ride
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle {radius: 2.0 }),
        Box::new(Square {side: 3.0 }),
    ];

}