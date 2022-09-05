struct Point {
    x: f64,
    y: f64
}

fn origin () -> Point {
    Point{x: 1.0, y: 0.2}
}

pub fn stack_and_heap() {
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 {} bayt yer kaplıyor.", std::mem::size_of_val(&p1));
    println!("p2 {} bayt yer kaplıyor.", std::mem::size_of_val(&p2));

    let p3 = *p2;

    println!("p3 {} bayt yer kaplıyor.", std::mem::size_of_val(&p3));
    println!("p3 değerler x: {}, y: {}", p3.x, p3.y);
    
}