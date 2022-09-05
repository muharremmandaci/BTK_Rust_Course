#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let mut a = 2+5*3;
    println!("{}",a);

    // a++ a-- Rust dilinde yoktur.
    a =  a + 1;
    a += 1; // -=, *=, /=, %= gibi

    println!("{}",a);

    /**************************/

    let cube_a = i32::pow(a, 3);
    println!("a'nı küpü : {}",cube_a);

    /**************************/
    let b = 2.6;
    let cube_b = f64::powi(b, 3);
    let power_pi_b = f64::powf(b, std::f64::consts::PI);

    println!("b({})'nin küpü -> powi: {}, powf : {}",b,cube_b, power_pi_b);
}
