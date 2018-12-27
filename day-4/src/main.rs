use std::fs;
use std::path::Path;

use chrono::DateTime;
use regex::Regex;

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).expect("YOLO");
    let re_datetime = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}").unwrap();

    let split_contents = contents.lines();

    for s in split_contents {
        println!("{}", s);
        let str_dt = re_datetime
            .captures(s)
            .unwrap()
            .get(0)
            .map_or("", |m| m.as_str());
        let dt = DateTime::parse_from_str(str_dt, "%F %H:%M");
        println!("{}", dt);
    }
}
