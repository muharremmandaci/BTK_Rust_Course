use std::collections::HashMap;
fn main() {
    let mut shapes = HashMap::new();

    let triangle = String::from("üçgen");
    shapes.insert(triangle, 3);
    shapes.insert("kare".into(), 4);

    println!("bir karenin {} kenarı vardır.", shapes["kare".into()]);

    for (k, v) in &shapes {
        println!("{}'nin {} kenarı vardır. ", k, v);
    }

    shapes.insert("kare".into(), 66); // değer güncellemesi
    println!("bir karenin {} kenarı vardır.", shapes["kare".into()]);

    println!("--------------------------");
    println!("{:?}", shapes);

    /********/

    shapes.entry("daire".into()).or_insert(444); // daire yoksa ekle
    shapes.entry("kare".into()).or_insert(444); // kare olduğu için değiştirmeyecek

    println!("{:?}", shapes);


    /********************/

    let temp = shapes.entry("yamuk".into()).or_insert(4);
    *temp = 0; // o değeri değiştirir.
    println!("{:?}", shapes);
}
