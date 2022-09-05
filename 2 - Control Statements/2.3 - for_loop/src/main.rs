fn main() {
    for x in 1..11 { // 1'den 11'e kadar 11 dahil değil

        if x == 3 {
            continue;
        } 
        else if x == 8 {
            break;
        }

        println!("x = {}",x);
    }

    // enumerate edince 0'dan başlayan indis ve numara gelecek
    for (poz, y) in (50..61).enumerate() {
        println!("indis: {}, değer: {}", poz, y);
    }
    /*
        indis: 0, değer: 50
        indis: 1, değer: 51
        indis: 2, değer: 52
        ...
    */



}
