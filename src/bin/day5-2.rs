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
            break;
        }
    }
    // merge intervals
    ranges.sort_by_key(|r| r.lower);
    let mut merged_ranges: Vec<Range> = Vec::new();
    for range in ranges {
        if merged_ranges.len() == 0 {
            merged_ranges.push(range);
            continue;
        }
        let last_range = merged_ranges.last_mut().unwrap();
        if range.lower >= last_range.lower && range.lower <= last_range.upper{
            // only update last range if the new one extends it, otherwise its fully contained
            if range.upper > last_range.upper {
                last_range.upper = range.upper
            }
        } else {
            merged_ranges.push(range);
        }

    }

    // add up all the ids from the ranges
    let mut total_ids = 0;
    for range in merged_ranges {
        total_ids += range.upper - range.lower + 1;
    }
    println!("total_ids {total_ids}");


}
