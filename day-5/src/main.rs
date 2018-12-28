use std::{fs, path::Path};

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents: String = fs::read_to_string(filename).unwrap();
}
