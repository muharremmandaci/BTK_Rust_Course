fn main() {
    let mut my_vec = vec![2,34,5];

    my_vec.push(6);

    for value in &my_vec {
        println!("{}", value);
    }

    for value in my_vec.iter() {
        println!("iter: {}", value);
    }

    for value in my_vec.iter().rev() {
        println!("reverse iter: {}", value);
    }

    for value in my_vec.iter_mut() {
        *value += 4;
    }

    println!("{:?}", my_vec);

    /******/

    let mut my_vec2 = vec![2,6,33];

    my_vec2.extend(my_vec); // vec2'ye vec'i ekledi
    println!("{:?}", my_vec2);

}
