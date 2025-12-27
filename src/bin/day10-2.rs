use AdventOfCode2025::utils::read_lines_into_vec;
use z3::{Optimize, SatResult, ast::*};

fn find_min_moves_for_joltages(buttons: &Vec<Vec<usize>>, target_joltages: &Vec<usize>) -> usize {
    // solve using z3, docs were a bit of a nightmare so I had to AI generate this
    let opt = Optimize::new();
    let num_variables = buttons.len();
    let mut variables = Vec::new();
    for i in 0..num_variables {
        let var = Int::new_const(format!("x{}", i));
        variables.push(var);
    }
    for i in 0..target_joltages.len() {
        let mut expr = Int::from_i64(0);
        for (j, button) in buttons.iter().enumerate() {
            if button.contains(&i) {
                expr = &expr + &variables[j];
            }
        }
        let target = Int::from_i64(target_joltages[i] as i64);
        opt.assert(&expr.eq(&target));
    }
    for var in &variables {
        opt.assert(&var.ge(&Int::from_i64(0)));
    }
    let sum_expr = variables.iter().fold(Int::from_i64(0), |acc, v| &acc + v);
    opt.minimize(&sum_expr);
    match opt.check(&[]) {
        SatResult::Sat => {
            let model = opt.get_model().unwrap();
            variables
                .iter()
                .map(|v| model.eval(v, true).unwrap().as_i64().unwrap() as usize)
                .sum()
        }
        _ => 0,
    }
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
        let paren_parts: Vec<&str> = parts[1..parts.len() - 1].to_vec();
        let brace_part = parts[parts.len() - 1];

        // println!("bracket: {}", bracket_part);
        // println!("parens: {:?}", paren_parts);
        // println!("brace: {}", brace_part);

        // iterate from 1 to length-1 of bracket part to get a set of indices that are '#'
        let mut buttons: Vec<Vec<usize>> = Vec::new();
        for paren_part in paren_parts {
            let nums: Vec<usize> = paren_part[1..paren_part.len() - 1]
                .split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect();
            buttons.push(nums);
        }

        let mut joltages: Vec<usize> = Vec::new();

        for c in brace_part[1..brace_part.len() - 1].split(',') {
            let joltage: usize = c.trim().parse().unwrap();
            joltages.push(joltage);
        }
        // println!("buttons: {:?}", buttons);
        // println!("joltages: {:?}", joltages);
        let moves = find_min_moves_for_joltages(&buttons, &joltages);
        println!("moves needed: {}", moves);
        total_moves += moves;
    }

    println!("total moves: {}", total_moves);
}
