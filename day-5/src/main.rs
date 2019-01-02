use num::abs;
use std::{fs, path::Path};

fn remove_pairs(seq: &mut Vec<u8>) {
    for i in (0..seq.len() - 1).rev() {
        if i >= seq.len() - 1 {
            continue;
        }

        if abs(seq[i + 1] as i8 - seq[i] as i8) == 0x20 {
            let a = seq.remove(i + 1) as char;
            let b = seq.remove(i) as char;
            assert_eq!(a.to_lowercase().to_string(), b.to_lowercase().to_string());
        }
    }
}

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let mut seq = contents.into_bytes();
    let mut prev_len = seq.len();

    loop {
        remove_pairs(&mut seq);

        if prev_len == seq.len() {
            break;
        }

        prev_len = seq.len();
    }

    println!("Number of units: {}", seq.len());
}
