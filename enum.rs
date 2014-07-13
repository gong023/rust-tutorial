struct Point {
    x: f64,
    y: f64
}

enum Shape {
    Circle(Point, f64),
    Rectangle(Point, Point)
}

fn main() {
    let my_point = Point { x: 1.0, y: 2.0 };
    Circle(Point { x: 1.0, y:2.0 }, 3.0);
    Rectangle(Point { x: 1.0, y: 2.0 }, my_point);
    area();
}

fn area(sh: Shape) -> f64 {
    match sh {
        Circle(_, size) => size * size,
        Rectangle(Point { x, y }, Point{ x: x2, y: y2 }) => (x + x2) * (y + y2)
    }
}
