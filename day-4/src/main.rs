use std::fs;
use std::path::Path;

use regex::Regex;

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).expect("YOLO");
    let re_date = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();

    println!("{}", contents);
}
