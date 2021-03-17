use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let lines = lines_from_file("input.txt");
    let mut valid_keys: Vec<&str> = Vec::new();
    let mut num_valid = 0;

    for line in &lines {
        if line == "" {
            println!("{:?}", valid_keys);
            if valid_keys.len() == required_keys.len() {
                num_valid += 1;
            }

            valid_keys = Vec::new();
            continue;
        }
        let pairs = line.split(" ");
        
        for pair in pairs {
            let mapping: Vec<&str> = pair.split(":").collect();
            if mapping[0] == "cid" {
                continue;
            }

            valid_keys.push(mapping[0].clone());
        }
    }

    println!("{:?}", valid_keys);
    if valid_keys.len() == required_keys.len() {
        num_valid += 1;
    }

    println!("Result: {:?}", num_valid);
}