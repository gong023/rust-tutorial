struct Point {
    x: f64,
    y: f64
}

fn main() {
    let mut mypoint = Point { x: 1.0, y: 1.0 };
    mypoint.x += 2.0;
    mypoint.y += 1.0;
    println!("{}", mypoint.x);
    point_match(mypoint);
}

fn point_match(p: Point) {
    println!("{}", p.y);
}
