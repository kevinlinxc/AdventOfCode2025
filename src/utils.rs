use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines_into_vec<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    // Collect the results into a vector, failing if any line read fails
    let lines_vec: Vec<String> = reader.lines().collect::<io::Result<_>>()?;
    Ok(lines_vec)
}
