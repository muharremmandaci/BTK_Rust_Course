// vektörler eleman sayısı değiştirilebilir veri tipidir.
fn main() {
    let mut my_vec: Vec<i32> = Vec::new();

    my_vec.push(1);
    my_vec.push(2);
    my_vec.push(3);
    my_vec.push(7);
    println!("{:?}", my_vec);

    /**/

    let mut my_vec2 = vec![1,2,3,4]; //[1; 10]'da kullanılabilir.
    println!("{:?}", my_vec2);

    println!("{:?}", &my_vec[0..2]); // & kullanmayınca compile time'da boyut bilinemez hatası

    match my_vec.get(3) {
        Some(x) => println!("var, {}",x),
        None => println!("değer yok")  
    }

    match my_vec.get(11) {
        Some(x) => println!("var {}",x),
        None => println!("değer yok")  
    }

    for val in &my_vec {
        println!("{}", val);
    }

    /******/

    let last_element = my_vec.pop();
    // last_element Some olarak dönecek.
    println!("son eleman: {:?}, ve vektörün kalan kısmı: {:?}", last_element, my_vec);

    // let Some(last_element) = my_vec.pop(); -> bu yöntem ile alındığında pop none'da dönebilir. some döneceği garanti olmadığı için hatalı

    // None dönerse while çalışmayacak !
    while let Some(x) = my_vec.pop() {
        println!("{}",x);
    }

    // yada

    let last_element2 = match my_vec2.pop() {
        Some(x) => println!("vec2 pop: {}",x),
        None => println!("None.")
    };

}
