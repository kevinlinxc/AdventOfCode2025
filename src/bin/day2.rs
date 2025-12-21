use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines_into_vec<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    // Collect the results into a vector, failing if any line read fails
    let lines_vec: Vec<String> = reader.lines().collect::<io::Result<_>>()?;
    Ok(lines_vec)
}

fn is_invalid_id(id: &u64) -> bool {
    // convert to string
    let str_id = id.to_string();
    let length = str_id.len();
    let first_half = str_id[0..length/2].to_string();
    let second_half = str_id[length/2..length].to_string();
    return first_half == second_half;
}

fn main() -> io::Result<()> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir).join("inputs").join("2.txt");
    
    let lines = read_lines_into_vec(file_name)?;
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
                println!("invalid: {i}");
                total += i;
            }

        }
    }
    
    println!("total: {total}");

    Ok(())
}
