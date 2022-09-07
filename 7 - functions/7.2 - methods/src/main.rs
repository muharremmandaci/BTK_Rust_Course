struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn calculate_len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;

        (dx * dx + dy * dy).sqrt()
    }
}


fn main() {
    let start_point = Point{x: 3.0, y: 4.0};
    let end_point = Point{x: 5.0, y: 10.0};

    let my_line = Line{start: start_point, end: end_point};

    let my_line_len = my_line.calculate_len();

    println!("mesafe: {}", my_line_len);
}
