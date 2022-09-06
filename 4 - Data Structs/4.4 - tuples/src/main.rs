
fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x+y,x*y)
}

fn main() {
    let x = 3;
    let y = 4;

    let results = sum_and_product(x, y);

    println!("{0} + {1} = {2} and {0} * {1} = {3}", x, y, results.0, results.1);

    let (result_sum, result_product) = results;

    println!("{0} + {1} = {2} and {0} * {1} = {3}", x, y, result_sum, result_product);


    // birden fazla veri tipi de içerebilir

    let my_tuple = (true, 23.1, -1i8);

    println!("{:?}", my_tuple);
}
