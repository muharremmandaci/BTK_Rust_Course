fn main() {
    let s = "merhaba"; // static UTF-8
    // immutable s = "hi" olarak atama yapılamaz.

    for c in s.chars() {
        println!("{}",c);
    }

    println!("--------------------------------------------------------");

    for c in s.chars().rev() {
        println!("{}",c);
    }

    println!("--------------------------------------------------------");

    if let Some(first_char) = s.chars().nth(0) {
        println!("ilk karakter: {}", first_char);
    }

    println!("------------------- STRING -----------------------------");

    // heap
    let mut chars = String::new();
    let mut a = 'a' as u8;

    while  a <= ('z' as u8) {
        chars.push(a as char);
        chars.push_str(",");
        a += 1;
    }

    println!("{:?}", chars);

    println!("--------------------------------------------------------");

    // str to String
    let u: &str = &chars;
    println!("{:?}", u);

    println!("--------------------------------------------------------");

    // String + str
    // String + &String gibi toplama yapılabilir

    let hello = String::from("Hello");
    let world = String::from(" World!");

    let mut hello_world = hello + &world;

    println!("{}",hello_world);

    println!("--------------------------------------------------------");

    hello_world.remove(0);

    println!("{}",hello_world);

    println!("--------------------------------------------------------");

    hello_world = hello_world.replace("o", "c");

    println!("{}",hello_world);

    println!("--------------------------------------------------------");

}
