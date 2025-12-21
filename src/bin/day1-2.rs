use AdventOfCode2025::utils::read_lines_into_vec;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("1.txt");

    let lines = read_lines_into_vec(file_name)?;

    let mut zero_count: i32 = 0;
    let mut lock_state: i32 = 50;
    for line in lines {
        // split into direction and magnitude
        let direction = &line[0..1];
        let magnitude: i32 = line[1..].trim().parse().unwrap();
        match direction {
            "R" => {
                for _ in 0..magnitude {
                    lock_state += 1;
                    if lock_state == 100 {
                        lock_state = 0;
                    }
                    if lock_state == 0 {
                        zero_count += 1;
                    }
                }
            }
            "L" => {
                for _ in 0..magnitude {
                    lock_state -= 1;
                    if lock_state == -1 {
                        lock_state = 99;
                    }
                    if lock_state == 0 {
                        zero_count += 1;
                    }
                }
            }
            _ => panic!("Unknown direction"),
        }

        println!("{}", line);
    }
    println!("Zero count: {}", zero_count);
}
