use AdventOfCode2025::utils::read_lines_into_vec;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn area(&self, other: &Point) -> usize {
        return (((self.x as i64 - other.x as i64).abs() + 1)
            * ((self.y as i64 - other.y as i64).abs() + 1)) as usize;
    }
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("9.txt");
    let lines = read_lines_into_vec(file_name).unwrap();
    let mut corners: Vec<Point> = Vec::new();
    for line in lines {
        let splitted: Vec<&str> = line.split(",").collect();
        let (x, y) = (splitted[0].parse().unwrap(), splitted[1].parse().unwrap());
        corners.push(Point{x:x, y:y});
    }
    let mut max_area = 0;
    let unique_tuples: Vec<(Point, Point)> = corners.iter().cloned().tuple_combinations().collect();
    for (point_a, point_b) in unique_tuples {
        let area = point_a.area(&point_b);
        if area > max_area {
            max_area = area;
        }
    }
    
    

    println!("max area: {:?}", max_area);
}
