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

    let mut seq = contents[..].to_string().into_bytes();
    let mut prev_len = seq.len();

    loop {
        remove_pairs(&mut seq);

        if prev_len == seq.len() {
            break;
        }

        prev_len = seq.len();
    }

    println!("Number of units: {}", seq.len());

    let mut c_remove = 'a';
    let mut shortest_polymer = seq.len();

    for c in ('a' as u8)..=('z' as u8) {
        let mut content_strip = contents[..].to_string();
        content_strip.retain(|s| !(s as u8 == c || (s as u8 + 0x20) == c));
        let mut seq = content_strip.into_bytes();

        remove_pairs(&mut seq);
        println!(
            "Number of units removing '{0}' is: {1}",
            c as char,
            seq.len()
        );

        if shortest_polymer > seq.len() {
            c_remove = c as char;
            shortest_polymer = seq.len();
        }
    }

    println!(
        "It is most effective to remove '{0}'. This yields a sequence of: {1} units",
        c_remove, shortest_polymer
    );
}
