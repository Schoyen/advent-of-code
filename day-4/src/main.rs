use std::path::Path;
use std::fs;

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename);

    println!("{:?}", contents);
}
