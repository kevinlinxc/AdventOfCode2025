use AdventOfCode2025::utils::read_lines_into_vec;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("3.txt");
    let lines = read_lines_into_vec(file_name).unwrap();

    let mut total_joltage: u32 = 0;
    // for each line, find the biggest digit up to and not including the last digit, and then the biggest digit after
    // the biggest found digit
    for line in lines {
        println!("{line}");
        // get length of the line
        let length = line.len();
        let mut biggest_index: usize = 0;
        let mut biggest_value: u32 = 0;
        for i in 0..length - 1 {
            // not including last digit?
            let current_value: u32 = line[i..i + 1].trim().parse().unwrap();
            if current_value > biggest_value {
                biggest_value = current_value;
                biggest_index = i;
            }
        }
        let mut biggest_after_biggest_value: u32 = 0;
        for i in biggest_index + 1..length {
            let current_value: u32 = line[i..i + 1].trim().parse().unwrap();
            if current_value > biggest_after_biggest_value as u32 {
                biggest_after_biggest_value = current_value;
            }
        }

        // combine the two biggest values into a string e.g. 2, 4 -> 24
        let combined_value_str = format!("{}{}", biggest_value, biggest_after_biggest_value);
        let combined_value: u32 = combined_value_str.trim().parse().unwrap();
        println!("Combined value: {}", combined_value);
        total_joltage += combined_value;

        println!("{}", line);
    }
    println!("Total joltage: {}", total_joltage);
}
