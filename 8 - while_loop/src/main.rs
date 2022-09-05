fn main() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 128 {
            continue; // while'ın bu iterasyonunu atlıyor.
        }

        println!("x = {}",x);
    }

    // return ile çıkana kadar sonsuz döngü oluşacak.
    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);

        if y > 1000 {
            break;
        }
    }
}
