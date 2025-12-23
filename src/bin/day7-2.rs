use AdventOfCode2025::utils::read_lines_into_vec;
use std::collections::HashMap;
use std::collections::HashSet;

fn pretty_print_map(lines: &Vec<String>, tachyon_locations: &Vec<(usize, usize)>) {
    println!("{}", String::from("-").repeat(15));
    let height = lines.len();
    let width = lines[0].len();
    for y in 0..height {
        for x in 0..width {
            if tachyon_locations.contains(&(x, y)) {
                print!("|")
            } else {
                print!("{}", &lines[y][x..x + 1]);
            }
        }
        println!("");
    }
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("7.txt");
    let lines = read_lines_into_vec(file_name).unwrap();
    let height = lines.len();
    let width = lines[0].len();
    // map of { y :  {x: quantity} } to avoid exponential
    let mut tachyon_map: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    let mut tachyon_locations: Vec<(usize, usize)> = Vec::new();
    let mut splitter_locations: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..width {
        for y in 0..height {
            match &lines[y][x..x + 1] {
                "." => continue,
                "S" => {
                    tachyon_map.insert(y, HashMap::from([(x, 1)]));
                }
                "^" => {
                    splitter_locations.insert((x, y));
                }
                c => panic!("{c}"),
            }
        }
    }

    for y in 0..height {
        if !tachyon_map.contains_key(&(y + 1)) {
            tachyon_map.insert(y + 1, HashMap::new());
        }
        let current_entries: Vec<(usize, usize)> =
            tachyon_map[&y].iter().map(|(x, q)| (*x, *q)).collect();
        for (x, quantity) in current_entries {
            if splitter_locations.contains(&(x, y + 1)) {
                // tachyon hit a splitter, add to left and right if possible
                if x >= 1 {
                    if let Some(inner_map) = tachyon_map.get_mut(&(y + 1)) {
                        if inner_map.contains_key(&(x - 1)) {
                            *inner_map.get_mut(&(x - 1)).unwrap() += quantity;
                        } else {
                            inner_map.insert(x - 1, quantity);
                        }
                    }
                }
                if x < width - 1 {
                    if let Some(inner_map) = tachyon_map.get_mut(&(y + 1)) {
                        if inner_map.contains_key(&(x + 1)) {
                            *inner_map.get_mut(&(x + 1)).unwrap() += quantity;
                        } else {
                            inner_map.insert(x + 1, quantity);
                        }
                    }
                }
            } else {
                // tachyon should continue down
                if let Some(inner_map) = tachyon_map.get_mut(&(y + 1)) {
                    if inner_map.contains_key(&x) {
                        *inner_map.get_mut(&x).unwrap() += quantity;
                    } else {
                        inner_map.insert(x, quantity);
                    }
                }
            }
        }
    }
    // add up all the tachyons in the final row

    let last_row = tachyon_map.get(&(width)).unwrap();
    let mut total_timelines = 0;
    for (x, quantity) in last_row {
        total_timelines += quantity;
    }
    println!("total_timelines : {total_timelines}");
}
