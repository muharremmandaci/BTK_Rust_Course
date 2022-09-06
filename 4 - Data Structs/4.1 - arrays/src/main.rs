use std::mem::size_of_val;
fn main() {
    let mut my_arr: [i32; 5] = [1,2,3,4,5];

    println!("my_arr eleman sayısı: {}, ilk elemanı: {}", my_arr.len(), my_arr[0]);

    my_arr[0] = 22;

    println!("{}", my_arr[0]);

    // assert_eq!(my_arr,[1,2,3,4,5]); bu hata döndürecektir. panic!
    assert_eq!(my_arr,[22,2,3,4,5]);

    if my_arr != [1,2,3,4,5] {
        println!("array değişmiş!");
    }

    /***************************/

    let b_arr = [1;10]; // 10 tane 1 içeren array

    for i in 0..b_arr.len() {
        println!("{}", b_arr[i]);
    }

    // tüm elemanları yazdırmak için
    println!("{:?}", b_arr);

    println!("b_arr dizisi {} bayt boyutundadır.", size_of_val(&b_arr));
}
