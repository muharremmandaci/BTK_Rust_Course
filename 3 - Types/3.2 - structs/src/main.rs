struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

fn main() {
    let start_point = Point{x: 4.0, y: 3.0};
    let end_point = Point{x: 5.0, y: 4.0};

    let line = Line{start: start_point, end: end_point};

    println!("line from ({}, {}) to ({}, {})", line.start.x, line.start.y, line.end.x, line.end.y);
}
