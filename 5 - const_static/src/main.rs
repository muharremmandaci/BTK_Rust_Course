// veri tipi  belirtilir.
// belirli bir adresi yoktur.
// mutable yapılamaz.
const PI: f32 = 3.14;

// veri tipi belirtilir
// belirli bir adresi vardır.
// static mutable yapılabilir.
static M: u8 = 10;
static mut N: u8 = 15;

fn main() {
    println!("PI : {}", PI);
    println!("M : {}", M);

    // değeri static olduğu halde değiştirilebilir.
    // ancak rust buna izin vermek istemediği için unsafe içerisinde yapılmalıdır.
    // const değişkenler için kesinlikle mut kullanılamaz.
    unsafe {
        N = 66;
        println!("N : {}", N);
    }
    
}
