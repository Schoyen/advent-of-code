use num::abs;
use std::{fs, path::Path};

struct Region {
    point: Point,
    area: u32,
}

impl Region {
    fn compute_area(&mut self, regions: &Vec<Region>) {
        for region in regions {
            if region.point.distance_from(&self.point) == 0 {
                continue;
            }
        }
    }
}

struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn distance_from(&self, other: &Point) -> u32 {
        (abs(self.x as i32 - other.x as i32) + abs(self.y as i32 - other.y as i32)) as u32
    }
}

fn get_point(str_tuple: &str) -> Point {
    let tup_vec: Vec<&str> = str_tuple.split(", ").collect();

    Point {
        x: tup_vec[0].parse().unwrap(),
        y: tup_vec[1].parse().unwrap(),
    }
}

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();
    let mut regions: Vec<Region> = Vec::new();
    let mut grid_size = 0;

    for line in contents.lines() {
        let point = get_point(line);

        if point.x > grid_size {
            grid_size = point.x;
        }

        if point.y > grid_size {
            grid_size = point.y;
        }

        regions.push(Region {
            point: point,
            area: 0,
        });
    }
}
