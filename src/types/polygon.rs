use std::{f64::consts::PI, fmt::Debug, ops::Add};

pub trait Circumference {
    fn circumference(&self) -> f64;
}

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    // add fields
    x: i32,
    y: i32,
}

impl Point {
    // add methods
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn magnitude(&self) -> f64 {
        f64::from(self.x.pow(2) + self.y.pow(2)).sqrt()
    }

    pub fn dist(&self, point_b: &Point) -> f64 {
        f64::from((self.x - point_b.x).pow(2) + (self.y - point_b.y).pow(2)).sqrt()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    // add methods
    pub fn new() -> Self {
        Self { points: vec![] }
    }

    pub fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }

    pub fn left_most_point(&self) -> Option<Point> {
        if self.points.is_empty() {
            return None;
        }
        self.points.iter().min_by_key(|item| item.x).cloned()
    }

    pub fn iter(&self) -> std::slice::Iter<Point> {
        self.points.iter()
    }
}

impl Circumference for Polygon {
    fn circumference(&self) -> f64 {
        let point_count = self.points.len();
        self.iter().enumerate().fold(0_f64, |mut acc, (idx, p)| {
            let next_point_idx = (idx + 1) % point_count;
            acc += p.dist(&self.points[next_point_idx]);
            acc
        })
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "center at: {:?}, radius is {}", self.center, self.radius)
    }
}

impl Circle {
    // add methods
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }
}

impl Circumference for Circle {
    fn circumference(&self) -> f64 {
        f64::from(self.radius * 2) * PI
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    pub fn circumference(&self) -> f64 {
        match self {
            Shape::Polygon(poly) => poly.circumference(),
            Shape::Circle(circle) => circle.circumference(),
        }
    }
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Self::Polygon(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Self::Circle(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(&p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1.clone());
        poly.add_point(p2.clone());
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_circumferences() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let circumferences = shapes
            .iter()
            .map(Shape::circumference)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(circumferences, vec![15.48, 31.42]);
    }
}
