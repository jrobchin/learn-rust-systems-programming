fn greet_world() {
    println!("Hello, world!");

    let french = "Salut le monde!";
    let spanish = "Hola Mundo!";
    let regions = [french, spanish];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}