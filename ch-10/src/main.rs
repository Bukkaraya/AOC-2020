use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i64> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.unwrap().parse().expect("Could not parse line"))
        .collect()
}

fn main() {
    let mut lines = lines_from_file("input.txt");
    lines.sort();
    let mut current_joltage = 0;
    let mut one_jolt_differences = 0;
    let mut three_jolt_differences = 0;


    while current_joltage <= lines[lines.len() - 1] {
        println!("Current {}", current_joltage);
        if lines.contains(&(current_joltage + 1)) {
            current_joltage += 1;
            one_jolt_differences += 1;
        } else if lines.contains(&(current_joltage + 2)) {
            current_joltage += 2;
        } else if lines.contains(&(current_joltage + 3)) {
            current_joltage += 3;
            three_jolt_differences += 1;
        } else {
            break;
        }
    }

    println!("{}", lines.len());
    println!("{}, {}", one_jolt_differences, three_jolt_differences + 1);
    println!("Result: {}", one_jolt_differences * (three_jolt_differences + 1));
}
