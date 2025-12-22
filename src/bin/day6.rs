use AdventOfCode2025::utils::read_lines_into_vec;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("6.txt");
    let lines = read_lines_into_vec(file_name).unwrap();
    let last_line = lines.last().unwrap();
    let operations: Vec<&str> = last_line.split_whitespace().collect();
    
    let mut number_lines: Vec<Vec<u64>> = Vec::new();
    for y in 0..lines.len() - 1 {
        let line = &lines[y];
        let numbers: Vec<u64> = line.split_whitespace().map(|x: &str| x.parse::<u64>().unwrap()).collect();
        number_lines.push(numbers);
    }

    let mut grand_total: u64 = 0;

    for x in 0..operations.len() {
        let operation = operations[x];
        let mut value: u64;
        if operation == "+" {
            value = 0;
        } else {
            value = 1;
        }
        for y in 0..number_lines.len(){
            match operation {
                "+" => {
                    value += number_lines[y][x];
                }
                "*" => {
                    value *= number_lines[y][x];
                }
                _ => {
                    panic!("poop");
                }
            }
        }
        grand_total += value;
    }

    println!("{grand_total}");
}
