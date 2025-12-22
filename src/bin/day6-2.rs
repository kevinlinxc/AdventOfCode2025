use AdventOfCode2025::utils::read_lines_into_vec;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("6.txt");
    let mut lines = read_lines_into_vec(file_name).unwrap();

    let last_line = lines.pop().unwrap();
    let operations: Vec<&str> = last_line.split_whitespace().collect();
    let width = lines[0].len();

    // we do a little transposing

    // make a vector of vectors, each inner vector contains characters and represents a column from the original
    let mut transposed: Vec<Vec<String>> = Vec::new();
    for i in 0..width {
        transposed.push(Vec::new());
    }

    for line in &lines {
        for x in 0..width {
            transposed[x].push(line[x..x + 1].to_string());
        }
    }

    // flatten the inner vecs into operation groups
    let mut operation_groups: Vec<Vec<u64>> = Vec::new();
    for _ in 0..operations.len() {
        operation_groups.push(Vec::new());
    }

    let mut current_operation_group: usize = 0;

    for x in 0..width{
        let column = &transposed[x];
        let joined: String = column.join("").trim().to_string();
        if joined.len() == 0 {
            // empty column, denotes we should move onto the next group
            current_operation_group += 1;
            continue
        } else {
            operation_groups[current_operation_group].push(joined.parse::<u64>().unwrap());
        }
    }

    let mut final_total: u64 = 0;

    for i in 0..operations.len() {
        println!("Doing operation {} on {:?}", operations[i], operation_groups[i]);
        match operations[i] {
            "+" => {
                let mut value = 0;
                for num in operation_groups[i].clone() {
                    value += num;
                }
                final_total += value;
            }
            "*" => {
                let mut value = 1;
                for num in operation_groups[i].clone() {
                    value *= num;
                }
                final_total += value;
            }
            _ => {
                panic!("poop");
            }
        }
    }

    println!("Final total: {final_total}")


}
