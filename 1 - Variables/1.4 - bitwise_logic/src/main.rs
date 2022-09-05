fn main() {
    // | -> or, & -> and, ^ -> xor 
    let c = 1|2; // 1->01 2->10 => 01 | 10 = 11 -> 3

    println!("{}",c);


    /*****************************/
    let pi_is_smaller_than_four = std::f64::consts::PI < 4.0;
    println!("{}",pi_is_smaller_than_four);

    let x = 5;
    let x_is_equal_to_five = x == 5;
    println!("{}",x_is_equal_to_five);
}
