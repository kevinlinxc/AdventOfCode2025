use AdventOfCode2025::utils::read_lines_into_vec;
use std::fmt;

struct Part {
    coords: Vec<(i32, i32)>,
    num_components: i64,
}

impl Part {
    fn new(coords: Vec<(i32, i32)>) -> Self {
        let num_components = coords.len() as i64;
        Self {
            coords,
            num_components,
        }
    }
}

impl fmt::Debug for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..3 {
            for x in 0..3 {
                if self.coords.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("12.txt");
    let lines = read_lines_into_vec(file_name).unwrap();
    for line in &lines {
        println!("{line}");
    }

    let mut parts_list: Vec<Part> = Vec::new();
    for i in 0..=5 {
        let line_0 = &lines[5 * i + 1];
        let line_1 = &lines[5 * i + 2];
        let line_2 = &lines[5 * i + 3];

        let mut coords = Vec::new();
        for (x, c) in line_0.chars().enumerate() {
            if c == '#' {
                coords.push((x as i32, 0 as i32));
            }
        }
        for (x, c) in line_1.chars().enumerate() {
            if c == '#' {
                coords.push((x as i32, 1 as i32));
            }
        }
        for (x, c) in line_2.chars().enumerate() {
            if c == '#' {
                coords.push((x as i32, 2 as i32));
            }
        }
        parts_list.push(Part::new(coords));
    }
    for (i, part) in parts_list.iter().enumerate() {
        println!("Part {}:\n{:?}", i, part);
    }

    println!("To assess:");
    let mut unsolvable_line_count = 0;
    let mut fit_line_count = 0;

    for i in 31..lines.len() {
        let line = &lines[i];
        println!("{line}");
        let parts = line.split(":").collect::<Vec<&str>>();
        let width_x_height = parts[0];
        let width_height_parts = width_x_height.split("x").collect::<Vec<&str>>();
        let width = width_height_parts[0].parse::<i32>().unwrap();
        let height = width_height_parts[1].parse::<i32>().unwrap();
        println!("Width: {}, Height: {}", width, height);
        let piece_count = parts[1]
            .trim()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        println!("Piece counts: {:?}", piece_count);

        // two easy cases: one, the total weighted sum of parts is greater than the area
        let total_area = width * height;
        let mut total_part_area = 0;
        let mut three_by_three_slots = 0;
        for (j, count) in piece_count.iter().enumerate() {
            let part = &parts_list[j];
            let part_area = part.num_components;
            total_part_area += part_area * (*count as i64);
            three_by_three_slots += count;
        }
        if total_part_area > total_area as i64 {
            println!(
                "Impossible to fit: total part area {} exceeds total area {}",
                total_part_area, total_area
            );
            continue;
        }
        // two, there is more than enough 3x3 slots to fit all the pieces
        let num_three_by_three_in_grid = (width / 3) * (height / 3);
        if num_three_by_three_in_grid >= three_by_three_slots {
            println!(
                "Possible to fit: enough 3x3 slots {} for all pieces {}",
                num_three_by_three_in_grid, three_by_three_slots
            );
            fit_line_count += 1;
            continue;
        }
        println!("Unsolvable line: {}", line);
        unsolvable_line_count += 1;
    }

    println!("Total unsolvable lines: {}", unsolvable_line_count);
    println!("Total fit lines: {}", fit_line_count);
}
