use AdventOfCode2025::utils::read_lines_into_vec;

#[derive(Debug)]
struct Range {
    lower: u64,
    upper: u64,
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("5.txt");
    let lines = read_lines_into_vec(file_name).unwrap();
    let mut ranges: Vec<Range> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    let mut parsing_ranges = true;
    for line in lines {
        if line == "" {
            parsing_ranges = false;
            continue;
        }

        if parsing_ranges {
            let (start, end) = line
                .split_once("-")
                .map(|(s, e)| (s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap()))
                .unwrap();

            ranges.push(Range {
                lower: start,
                upper: end,
            })
        } else {
            ids.push(line.parse::<u64>().expect("poop"))
        }
    }
    let mut num_fresh_ids = 0;

    for id in ids {
        for range in &ranges {
            if id >= range.lower && id <= range.upper {
                num_fresh_ids += 1;
                break;
            }
        }
    }
    println!("fresh ids: {num_fresh_ids}");
}
