use std::f64::consts::PI;

trait Polygon {
    // method to render a shape
    fn render(&self);
    type AreaType;
    fn area(&self) -> Self::AreaType;
}

#[derive(Debug)]
struct Square {
    side: i32,
}
impl Polygon for Square {
    type AreaType = i32;

    // renders Square
    fn render(&self) {
        println!("Rendering Square... {}", &self.area());
    }

    fn area(&self) -> i32 {
        let _area = self.side * self.side;
        return _area;
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}
impl Polygon for Circle {
    type AreaType = f64;

    // renders circle
    fn render(&self) {
        println!("Rendering Circle... {:.1$}", &self.area().to_string(), 4);
    }

    fn area(&self) -> f64 {
        let _area = self.radius.powf(2.0) * PI;
        return _area;
    }
}

pub fn simulate() {
    // create an object of Square
    let square: Square = Square { side: 10 };
    square.render();

    // create an object of Circle
    let circle: Circle = Circle { radius: 5.0 };
    circle.render();
}
