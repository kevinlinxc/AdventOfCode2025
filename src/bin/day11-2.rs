use AdventOfCode2025::utils::read_lines_into_vec;
use std::collections::HashMap;

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    end: &str,
    memo: &mut HashMap<(String, String), i64>, // cache how many paths between any (start, end)
) -> i64 {
    if current == end {
        return 1;
    }
    let key = (current.to_string(), end.to_string());
    if let Some(&val) = memo.get(&key) {
        return val;
    }
    let mut total = 0;
    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            total += count_paths(graph, neighbor, end, memo);
        }
    }
    memo.insert(key, total);
    total
}

fn main() {
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("11.txt");
    let lines = read_lines_into_vec(file_name).unwrap();
    for line in lines {
        println!("{}", line);

        let parts = line.split(": ").collect::<Vec<&str>>();
        let source = parts[0].to_string();
        let targets = parts[1]
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        adjacency.insert(source.clone(), targets.clone());

        println!("source: {}", source);
        println!("targets: {:?}", targets);
    }
    println!("{:?}", adjacency);
    // adjacency.insert("out".to_string(), Vec::new());
    let mut memo = HashMap::new();
    let count_svr_fft = count_paths(&adjacency, "svr", "fft", &mut memo);
    println!("count_svr_fft: {:?}", count_svr_fft);
    let count_fft_dac = count_paths(&adjacency, "fft", "dac", &mut memo);
    println!("count_fft_dac: {:?}", count_fft_dac);
    let count_dac_out = count_paths(&adjacency, "dac", "out", &mut memo);
    println!("count_dac_out: {:?}", count_dac_out);
    let count_svr_dac = count_paths(&adjacency, "svr", "dac", &mut memo);
    println!("count_svr_dac: {:?}", count_svr_dac);
    let count_dac_fft = count_paths(&adjacency, "dac", "fft", &mut memo);
    println!("count_dac_fft: {:?}", count_dac_fft);
    let count_fft_out = count_paths(&adjacency, "fft", "out", &mut memo);
    println!("count_fft_out: {:?}", count_fft_out);
    let count = count_svr_fft * count_fft_dac * count_dac_out
        + count_svr_dac * count_dac_fft * count_fft_out;
    println!("paths through dac and fft: {:?}", count);
    let count2 = count_paths(&adjacency, "svr", "out", &mut memo);
    println!("total paths: {:?}", count2);
}
