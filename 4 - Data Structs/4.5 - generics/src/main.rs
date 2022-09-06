struct Point {
    x: f64,
    y: f64
}

struct GenericPoint <T, V> {
    x: T,
    y: V
}

struct Line <T, V> {
    start: GenericPoint<T,V>,
    end: GenericPoint<T,V>
}

fn main() {
    let my_point = Point{x: 4.0, y: 7.0};

    let my_generic_point = GenericPoint{x: 4.0, y: 3};
    let my_generic_point2 = GenericPoint{x: "a", y: "b"}; // anlamsız ama oluyor.
}
