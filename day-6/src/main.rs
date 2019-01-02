use std::{fs, path::Path};
use ndarray::prelude::Array;

fn generate_grid(tuples: &str, size: u32) -> Array {
    let mut grid = Array::zeros(());
}

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let mut num_points = 0;

    for line in contents.lines() {
        num_points += 1;
        println!("{}", line);
    }

    println!("{}", num_points);
}
