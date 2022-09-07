use rand::Rng;
use rand::thread_rng;
use std::io::Read;
use std::io::stdin;

fn main() {
    let number = thread_rng().gen_range(1, 101);

    loop {
        println!("Tahmin Girin: ");
        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(input) => {
                let parsed = buffer.trim_end().parse::<i64>(); // sonuna boşluk koyduysa silecek

                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("1-100 arası değer girin.");
                        }
                        else if guess > number {
                            println!("tahmininiz yüksek");
                        }
                        else if guess < number {
                            println!("tahmininiz düşük");
                        }
                        else {
                            println!("doğru tahmin");
                            break;
                        }
                    },

                    Err(err) => {
                        println!(" okunamadı. {}. Tekrar dene", err);
                    }
                }
            },
            Err(_) => {
                continue;
            }
        }
    }
}
