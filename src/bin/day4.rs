use AdventOfCode2025::utils::read_lines_into_vec;

fn check_grid(grid: &Vec<String>, x: usize, y: usize, height: usize, width: usize) -> bool {
    if grid[y][x..x + 1].to_string() != "@" {
        return false;
    }
    // check 8 directions
    let mut taken_spots = 0;
    if x >= 1 {
        // we can look left
        // just left
        taken_spots += (grid[y][x - 1..x].to_string() == "@") as u64;

        if y >= 1 {
            // top left
            taken_spots += (grid[y - 1][x - 1..x].to_string() == "@") as u64;
        }

        if y < height - 1 {
            // bottom left
            taken_spots += (grid[y + 1][x - 1..x].to_string() == "@") as u64;
        }
    }
    // straight up
    if y >= 1 {
        taken_spots += (grid[y - 1][x..x + 1].to_string() == "@") as u64;
    }
    // straight down
    if y < height - 1 {
        taken_spots += (grid[y + 1][x..x + 1].to_string() == "@") as u64;
    }

    // right
    if x < width - 1 {
        // just right
        taken_spots += (grid[y][x + 1..x + 2].to_string() == "@") as u64;

        // top right
        if y >= 1 {
            taken_spots += (grid[y - 1][x + 1..x + 2].to_string() == "@") as u64;
        }

        // bottom right

        if y < height - 1 {
            taken_spots += (grid[y + 1][x + 1..x + 2].to_string() == "@") as u64;
        }
    }

    return taken_spots < 4;
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("4.txt");
    let lines = read_lines_into_vec(file_name).unwrap();
    let height = lines.len();
    let width = lines[0].len();
    let mut total_spots = 0;
    for i in 0..width {
        for j in 0..height {
            if check_grid(&lines, i, j, height, width) {
                println!("{i},{j}");
                total_spots += 1;
            }
        }
    }
    println!("total spots: {total_spots}");
}
