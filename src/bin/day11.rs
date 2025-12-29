use AdventOfCode2025::utils::read_lines_into_vec;
use std::collections::{HashMap, HashSet};

fn all_paths(graph: &HashMap<String, Vec<String>>, start: String, end: String) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    let mut path = vec![start.clone()];
    let mut visited = HashSet::new();

    dfs(&start, &end, graph, &mut visited, &mut path, &mut paths);
    return paths;
}

fn dfs(
    current: &String,
    end: &String,
    graph: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    path: &mut Vec<String>,
    paths: &mut Vec<Vec<String>>,
) {
    if *current == *end {
        paths.push(path.clone());
        return;
    }

    visited.insert(current.clone());

    for node in &graph[current] {
        if !visited.contains(node) {
            path.push(node.clone());
            dfs(&node, end, graph, visited, path, paths);
            path.pop();
        }
    }

    visited.remove(current);
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

    let paths = all_paths(&adjacency, "you".to_string(), "out".to_string());
    println!("{:?}", paths.len());
}
