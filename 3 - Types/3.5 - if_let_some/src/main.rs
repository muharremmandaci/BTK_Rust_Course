fn main() {
    let x: f32 = 2.0;
    let y: f32 = 0.0;

    // option -> Some(c) | None ikisinden birini
    let result = if y != 0.0 {
        Some(x/y)
    }
    else {
        None
    };

    match result {
        Some(z) => println!("{}/{}={}",x,y,z),
        None => println!("sıfıra bölme durumu")
    }
}
