fn main() {
    let temperature = 20;

    if temperature > 35 {
        println!("Çok sıcak!");
    }
    else if temperature < 10 {
        println!("Hava soğuk!")
    }
    else {
        println!("Hava normal.")
    }

    // bu tip bir if else yapısı kullanılarak ifadeye bağlı return yapılabilir.
    let day = if temperature > 20 {"güneşli"} else {"bulutlu"};
    println!("gün {}", day);

    println!("hava durumu {}",
        if temperature > 20 {"sıcak"} else if temperature < 10 {"soğuk"} else {"normal"});
}
