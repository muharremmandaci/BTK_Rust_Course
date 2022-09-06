union IntOrFloat {
    i:i32,
    f:f32
}
fn main() {
    let mut iof = IntOrFloat{i:122};

    iof.i = 221;

    let value = unsafe{ iof.i };

    println!("{}", value);

    // bu kodda sürekli unsafe kullanmış, mantıklı bir anlatım olmamış :) :P
}
