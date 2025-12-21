use AdventOfCode2025::utils::read_lines_into_vec;

fn is_invalid_id(id: &u64) -> bool {
    // iterate from 1 to length/2
    // check if the rest of the string is copies of the substring
    // convert to string
    let str_id = id.to_string();
    let length = str_id.len();
    for i in 1..=length / 2 {
        if length % i == 0 {
            let sub_str = str_id[0..i].to_string();
            let mut all_good = true;
            for j in (i..length).step_by(i) {
                if str_id[j..j + i] != sub_str {
                    all_good = false;
                }
            }
            if all_good {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("2.txt");

    let lines = read_lines_into_vec(file_name).unwrap();
    let first_line = lines[0].clone();
    // split on commas
    let intervals: Vec<&str> = first_line.split(',').collect();
    println!("{:?}", intervals);
    let mut total = 0;
    for interval in intervals {
        // split on -
        let (start_str, end_str) = interval.split_once("-").expect("poop");
        let start: u64 = start_str.parse::<u64>().expect("poop");
        let end: u64 = end_str.parse::<u64>().expect("poop");
        for i in start..=end {
            if is_invalid_id(&i) {
                println!("{i} is invalid");
                total += i;
            }
        }
    }

    println!("total: {total}");
}
