fn use_slice(slice:&mut[i32]) {
    println!("first elements slice: {}, and len: {}", slice[0], slice.len());

    slice[0]=33;
}
fn main() {
    let mut data = [1,2,3,4,5];

    use_slice(&mut data[1..4]); // 1,2 ve 3. eleman

    println!("{:?}",data); // slice fonksiyonda değiştirildiğinde data da değişmektedir. adresi gönderiliyor !!!
}
