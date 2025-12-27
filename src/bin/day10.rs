use AdventOfCode2025::utils::read_lines_into_vec;
use std::collections::HashSet;

fn bfs_min_moves(target_state: &HashSet<usize>, buttons: &Vec<Vec<usize>>) -> usize {
    let state: HashSet<usize> = HashSet::new(); // initial state is all off
    let mut visited: HashSet<Vec<usize>> = HashSet::new();
    let mut current_depth: Vec<HashSet<usize>> = Vec::new();

    let state_as_vec = state.clone().iter().cloned().collect::<Vec<usize>>();
    current_depth.push(state.clone());
    visited.insert(state_as_vec);
    let mut depth = 0;
    loop {
        depth += 1;
        let mut next_depth: Vec<HashSet<usize>> = Vec::new();
        for current_state in current_depth {
            for button in buttons {
                let mut new_state = current_state.clone();
                for &index in button {
                    if new_state.contains(&index) {
                        new_state.remove(&index);
                    } else {
                        new_state.insert(index);
                    }
                }
                if new_state == *target_state {
                    return depth;
                }
                let new_state_as_vec = new_state.clone().iter().cloned().collect::<Vec<usize>>();
                if !visited.contains(&new_state_as_vec) {
                    visited.insert(new_state_as_vec);
                    next_depth.push(new_state);
                }
            }
        }
        if next_depth.is_empty() {
            break;
        }
        current_depth = next_depth;
    }

    // TODO: implement BFS to compute minimal moves
    0
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_name = std::path::Path::new(manifest_dir)
        .join("inputs")
        .join("10.txt");
    let lines = read_lines_into_vec(file_name).unwrap();
    let mut total_moves = 0;
    for line in lines {
        println!("{}", line);
        // example line is like [..#..] (0,3,4) (0,1,4) (3,4) (2,3,4) (1,2) {158,164,29,36,181}
        let parts: Vec<&str> = line.split(' ').collect();
        let bracket_part = parts[0];
        let paren_parts: Vec<&str> = parts[1..parts.len() - 1].to_vec();
        let brace_part = parts[parts.len() - 1];

        println!("bracket: {}", bracket_part);
        println!("parens: {:?}", paren_parts);
        println!("brace: {}", brace_part);

        // iterate from 1 to length-1 of bracket part to get a set of indices that are '#'
        let mut hash_indices: HashSet<usize> = HashSet::new();
        let mut buttons: Vec<Vec<usize>> = Vec::new();
        for paren_part in paren_parts {
            let nums: Vec<usize> = paren_part[1..paren_part.len() - 1]
                .split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect();
            buttons.push(nums);
        }
        let mut joltages: HashSet<u32> = HashSet::new();
        for i in 1..bracket_part.len() - 1 {
            if &bracket_part[i..i + 1] == "#" {
                hash_indices.insert(i - 1);
            }
        }

        for c in brace_part[1..brace_part.len() - 1].split(',') {
            let joltage: u32 = c.trim().parse().unwrap();
            joltages.insert(joltage);
        }
        println!("target state: {:?}", hash_indices);
        println!("buttons: {:?}", buttons);
        println!("joltages: {:?}", joltages);

        let moves = bfs_min_moves(&hash_indices, &buttons);
        total_moves += moves;
    }

    println!("total moves: {}", total_moves);
}
