use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let filename = "input.txt";
    let year = 2020;

    let buffer = BufReader::new(File::open(filename).expect("no such file"));
    
    let lines: Vec<i64> = buffer.lines()
        .map(|l| l.expect("Could not parse line").parse().unwrap())
        .collect();


    let set: HashSet<i64> = HashSet::from_iter(lines.iter().cloned());
    let mut found = false;

    for number in &lines {
        let delta = year - number;
        for second_number in &lines {
            let delta = delta - second_number;
            if set.contains(&delta) {
                println!("Answer: {}", number * second_number * delta);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
}