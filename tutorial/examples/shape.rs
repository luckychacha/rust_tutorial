trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    // let shapes: Vec<&dyn Shape> = vec![
    //     Box::new(Circle { radius: 5.0 }),
    //     Box::new(Rectangle {
    //         width: 3.0,
    //         height: 4.0,
    //     }),
    // ];

    let shapes: Vec<&dyn Shape> = vec![
        &Circle { radius: 5.0 },
        &Rectangle {
            width: 3.0,
            height: 4.0,
        },
    ];

    for shape in shapes {
        println!("Area: {}", shape.area());
    }
}
