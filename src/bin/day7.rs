use AdventOfCode2025::utils::read_lines_into_vec;
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
    let mut tachyon_locations: Vec<(usize, usize)> = Vec::new();
    let mut tachyon_history: HashSet<(usize, usize)> = HashSet::new();
    let mut splitter_locations: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..width {
        for y in 0..height {
            match &lines[y][x..x + 1] {
                "." => continue,
                "S" => tachyon_locations.push((x, y)),
                "^" => {
                    splitter_locations.insert((x, y));
                }
                c => panic!("{c}"),
            }
        }
    }

    let mut changes_made = 1;
    let mut total_splits = 0;
    while changes_made != 0 {
        changes_made = 0;
        pretty_print_map(&lines, &tachyon_locations);
        let mut new_tachyon_locations: Vec<(usize, usize)> = Vec::new();
        for tachyon_location in tachyon_locations.clone() {
            // go one spot down, if its a ., add a tachyon there,
            // if its a splitter, add one to left and right of the splitter and increment splits
            let (x, y) = tachyon_location;
            if y == height - 1 {
                continue;
            }
            if &lines[y + 1][x..x + 1] == "." {
                if !tachyon_history.contains(&(x, y + 1)) {
                    new_tachyon_locations.push((x, y + 1));
                    tachyon_history.insert((x, y + 1));
                }
                changes_made += 1;
            } else if splitter_locations.contains(&(x, y + 1)) {
                total_splits += 1;
                println!("Splitting at {},{}", y + 1, x);
                if x >= 1 {
                    changes_made += 1;
                    if !tachyon_history.contains(&(x - 1, y + 1)) {
                        new_tachyon_locations.push((x - 1, y + 1));
                        tachyon_history.insert((x - 1, y + 1));
                    }
                }

                if x < width - 1 {
                    if !tachyon_history.contains(&(x + 1, y + 1)) {
                        new_tachyon_locations.push((x + 1, y + 1));
                        tachyon_history.insert((x + 1, y + 1));
                    }
                    changes_made += 1;
                }
            }
            tachyon_locations = new_tachyon_locations.clone();
        }
    }
    println!("total splits: {total_splits}");
}
