fn main() {
    let name = "Muharrem";
    let hello = format!("merhaba ben {}",name);

    println!("{}", hello);

    println!("----------------------------------------------------");

    let hi = "Selam";
    let rust = "Rust";

    let hi_rust = format!("{} {}", hi, rust);

    println!("{}", hi_rust);

    println!("----------------------------------------------------");

    let run = "run";
    let forest = "forest";
    let rfr = format!("{0} {1} {0}", run, forest);

    println!("{}", rfr);

    println!("----------------------------------------------------");

    let info = format!(
        "Hello, I'm {first} {end}",
        first = "James",
        end = "Bond"
    );

    println!("{}", info);

    println!("----------------------------------------------------");

    let mixed = format!(
        "{1}, {}, {0}, {}, {data}", // beta, alpha, alpha, beta, gamma
        "alpha",
        "beta",
        data = "gamma"
    );

    println!("{}", mixed);
}
