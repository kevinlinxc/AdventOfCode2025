use AdventOfCode2025::utils::read_lines_into_vec;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;
        let dz = self.z as f64 - other.z as f64;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("8.txt");
    let lines = read_lines_into_vec(file_name).unwrap();
    let num_boxes = lines.len();
    let mut boxes: Vec<Point> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(",").collect();
        let (x, y, z) = (parts[0], parts[1], parts[2]);
        boxes.push(Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        });
    }
    // calculate all pairwise
    let mut distances: Vec<((Point, Point), f64)> = Vec::new();
    let unique_tuples: Vec<(Point, Point)> = boxes.iter().cloned().tuple_combinations().collect();
    for (point_a, point_b) in unique_tuples {
        distances.push(((point_a, point_b), point_a.distance(&point_b)));
    }

    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    // for distance in &distances {
    //     println!("{:?} - {:?}: {}", distance.0.0, distance.0.1, distance.1);
    // }
    // make a map of boxes: what circuit they are in
    let mut box_circuit_lookup: HashMap<Point, usize> = boxes
        .iter()
        .enumerate()
        .map(|(i, &point)| (point, i))
        .collect();
    // also make a map of circuits: what boxes are in them
    let mut circuit_boxes: HashMap<usize, Vec<Point>> =
        (0..num_boxes).map(|i| (i, vec![boxes[i]])).collect();

    let num_connections = 1000;

    for i in 0..num_connections {
        let connection = &distances[i];
        let point_a = connection.0.0;
        let point_b = connection.0.1;
        // connect point b to a by...
        // 1. getting a's circuit
        // 2. getting b's circuit
        let circuit_a = box_circuit_lookup[&point_a];
        let circuit_b = box_circuit_lookup[&point_b];
        if circuit_a == circuit_b {
            continue;
        }
        println!(
            "Connecting {:?} (circuit {:?}) and {:?} (circuit {:?})",
            point_a, circuit_a, point_b, circuit_b
        );

        // 3. assign everything in b's circuit to be in a's circuit
        // 4. update circuit_boxes so that everything that was in b's circuit is now in a's circuit
        while circuit_boxes[&circuit_b].len() != 0 {
            let popped_box = circuit_boxes.get_mut(&circuit_b).unwrap().pop().unwrap();
            *box_circuit_lookup.get_mut(&popped_box).unwrap() = circuit_a;
            circuit_boxes.get_mut(&circuit_a).unwrap().push(popped_box);
        }
    }

    // at the end, sort circuit_boxes by length of vectors, and print them all out
    let mut circuits: Vec<_> = circuit_boxes.values().collect();
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    for circuit in &circuits {
        println!("Circuit size: {}", circuit.len());
    }

    // multiply top 3 biggest circuits
    let final_num = &circuits[0].len() * &circuits[1].len() * &circuits[2].len();
    println!("final: {final_num}");
}
