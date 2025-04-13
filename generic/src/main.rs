struct Point<T> {
    x: T,
    y: T,
}
impl Point<i32> {
    fn distance_from_origin(&self) -> i32 {
        self.x + self.y
    }
}
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    let point = Point { x: 3, y: 4 };
    println!("distance for i32 is {}", point.distance_from_origin());
    let point = Point { x: 3.0, y: 4.0 };
    println!("distance for f64 is {}", point.distance_from_origin());
}
