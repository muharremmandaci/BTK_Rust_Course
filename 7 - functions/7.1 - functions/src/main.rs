fn base_func() {
    println!("base function");
}

fn print_value(x: i32) {
    println!("value: {}", x);
}

fn increment_value(x: &mut i32) {
    *x += 1;
}

fn product_values(x: i32, y: i32) -> i32 {
    x*y
}

fn main() {
    base_func();
    
    println!("---------------------------------------------");

    let value: i32 = 5;

    print_value(value);

    println!("---------------------------------------------");

    let mut z = 22;

    increment_value(&mut z);
    println!("{}", z);

    println!("---------------------------------------------");

    let a = 5;
    let b = 7;

    let result = product_values(a, b);
    println!("çarpma sonucu: {}",result);

    println!("{}",a);
}
