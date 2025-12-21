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

fn main() -> io::Result<()> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir).join("inputs").join("1.txt");
    
    let lines = read_lines_into_vec(file_name)?;
    
    let mut zero_count: i32 = 0;
    let mut lock_state: i32 = 50;
    for line in lines {
        // split into direction and magnitude
        let direction = &line[0..1];
        let magnitude: i32 = line[1..].trim().parse().unwrap();
        match direction {
            "R" => lock_state += magnitude,
            "L" => lock_state -= magnitude,
            _ => panic!("Unknown direction"),
        }
        lock_state %= 100;
        if lock_state == 0 {
            zero_count += 1;
        }
        println!("{}", line);
    }
    println!("Zero count: {}", zero_count);

    Ok(())
}
