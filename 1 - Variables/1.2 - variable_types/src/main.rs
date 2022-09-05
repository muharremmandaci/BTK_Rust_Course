#[allow(dead_code)] // işlevi olmayan kodlar için gelen uyarıları kapatır
#[allow(unused_variables)] // kullanılmayan değişkenler için gelen uyarıları kapatır

use std::mem::size_of_val;

fn main() {

    /******************************/
    let a: u8 = 125; 
    println!("a = {}", a);

    /******************************/
    let b: i8 = 0;
    println!("b = {}", b);

    // b = 44; b immutable olduğu için hata verir

    /******************************/
    let mut c: i16 = -256;
    c = -268;

    println!("c = {}", c);

    /******************************/
    let d = 123456789;
    println!("d = {} ve d'nin boyutu {} bayttır.", d, size_of_val(&d));

    /******************************/
    
    let e:isize = -200; // isize,usize pc mimarisine göre boyut ataması yapar.
    let size_e = size_of_val(&e);
    println!("e = {} ve boyutu {}, bilgisayarınız {} bit mimariye sahiptir.", e, size_e, size_e*8);

    /************ CHAR ******************/
    let f:char = 'g';
    let size_f = size_of_val(&f);
    println!("f = '{}' ve boyutu {}, bilgisayarınız {} bit mimariye sahiptir.", f, size_f, size_f*8);

    /************ FLOAT ******************/
    // f32 ve f64 olabilir. unsigned olamaz. IEEE754 ile nan, +- sonsuz değerleri de alabilir. default f64 olur.

    let float_number: f32 = 2.0005;
    let size_float_number = size_of_val(&float_number);
    println!("float_number = '{}' ve boyutu {} bayt", float_number, size_float_number);

    /************ BOOLEAN ******************/
    // bool 1 byte yer işgal eder.
    let bool_value: bool = true;
    let size_bool_value = size_of_val(&bool_value);
    println!("bool_value = '{}' ve boyutu {} bayt", bool_value, size_bool_value);
}
