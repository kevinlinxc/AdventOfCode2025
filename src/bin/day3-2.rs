use AdventOfCode2025::utils::read_lines_into_vec;

fn vec_string_to_number(vec: &Vec<String>) -> u64 {
    println!("Vec: {:?}", vec);
    let combined_str = vec.join("");
    return combined_str
        .trim()
        .parse()
        .expect("Failed to parse combined string to number");
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("3.txt");
    let lines = read_lines_into_vec(file_name).unwrap();

    let mut total_joltage: u64 = 0;
    // start with a left pointer at 0 and a right pointer at length - characters_remaining (12 to start)
    // find the biggest digit between the pointers. Set the left pointer to this, and move the right pointer by one
    // iterate until we have 12 characters
    for line in lines {
        let mut all_chars: Vec<String> = Vec::new();
        let length = line.len();
        let mut left_pointer: usize = 0;
        let mut right_pointer: usize = length - (12 - all_chars.len());
        while all_chars.len() < 12 {
            let mut biggest_index: usize = left_pointer;
            let mut biggest_value: u64 = 0;
            for i in left_pointer..=right_pointer {
                let current_value: u64 = line[i..i + 1].trim().parse().unwrap();
                if current_value > biggest_value {
                    biggest_value = current_value;
                    biggest_index = i;
                }
            }
            all_chars.push(biggest_value.to_string());
            left_pointer = biggest_index + 1;
            right_pointer += 1;
        }
        println!("{line}");
        let combined_value = vec_string_to_number(&all_chars);
        total_joltage += combined_value;
        println!("Combined value: {}", combined_value);
    }
    println!("Total joltage: {}", total_joltage);
}
