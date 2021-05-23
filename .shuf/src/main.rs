use rand::seq::IteratorRandom;
use std::fs;

fn main() {
    let paths = fs::read_dir(".").unwrap();
    let mut rng = rand::thread_rng();
    let output = paths.choose(&mut rng).unwrap();
    println!("{}", output.unwrap().file_name().into_string().unwrap());
}
