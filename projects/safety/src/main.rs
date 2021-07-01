use std::thread;

#[derive(Debug)]
enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn dangling_pointer() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    drop(grains);

    println!("{:?}", grains);
}

fn data_race() {
    let mut data = 100;

    thread::spawn(|| { data = 500; });
    thread::spawn(|| { data = 1000; });
    
    println!("{}", data);
}

fn main() {
    dangling_pointer();
    data_race();
}
