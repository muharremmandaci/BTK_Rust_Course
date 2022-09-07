fn is_even(x: u32) -> bool {
    x%2 == 0
}

fn foo() {
    let limit = 500;
    let mut sum = 0;

    let to_limit = |y: u32| y > limit;

    for i in 0.. {
        let value: u32 = i*i;

        if to_limit(value) {
            break;
        }
        else if is_even(value) {
            sum += value;
        }

        println!("döngünün toplamı: {}", sum);
    }

    
}

fn main() {
    foo();
}
