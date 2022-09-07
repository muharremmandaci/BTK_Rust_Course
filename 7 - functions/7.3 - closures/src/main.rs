fn say_hello() {
    println!("Hello!");
}
fn main() {
    say_hello();
    let say_hi = say_hello;

    say_hi();

    println!("-------------------------------------------------");

    let increase_one = |x:i32| -> i32 {
        x+1
    };

    let mut number = 5;

    number = increase_one(number);

    println!("{}", number);

    println!("-------------------------------------------------");

    let mut number2 = 2;

    let increase_three = |x: &mut i32| *x += 3;
    increase_three(&mut number2);

    println!("{}", number2);
}
