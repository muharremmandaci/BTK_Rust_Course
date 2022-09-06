enum Color {
    Red,
    Blue,
    Green,
    RGBColor(u8, u8, u8), // tuple
    Cmyk{cyan: u8, magenta: u8, yellow: u8, black: u8} // struct
}

fn main() {
    let c: Color = Color::Blue;
    let d = Color::Red;
    let e: Color = Color::Cmyk { cyan: 10, magenta: 15, yellow: 20, black: 25 };
    let f: Color = Color::RGBColor(10, 20, 30);

    match e {
        Color::Blue => println!("b"),
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::RGBColor(_,_,_) => println!("RGBColor"),
        Color::Cmyk { cyan:_, magenta:_, yellow:_, black:_ } => println!("Cmyk")
    }

    match f {
        Color::Blue => println!("b"),
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::RGBColor(r, g, b) => println!("{}, {}, {}",r,g,b),
        Color::Cmyk { cyan:_, magenta:_, yellow:_, black:_ } => println!("Cmyk")
    }
}
