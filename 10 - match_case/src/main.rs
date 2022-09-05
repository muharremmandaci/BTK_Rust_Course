
fn main() {
    let country_code = 46;

    // match tüm değerleri kapsamalıdır, _ => satırı silinirse hata verecektir. Boşta değer bırakılmamalıdır!
    let country = match country_code {
        44 => "UK",
        46 => "Sweeden",
        7  => "Russia",
        90 => "Turkey",
        1..=1000 => "Unknown",
        _ => "Invalid"
    };

    println!("the country code {} is {}", country_code, country);

    let x = false;

    let value = match x {
        false => "Yanlış",
        true => "Doğru"
    };

    println!("value: {}", value);
}
