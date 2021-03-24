use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};
use std::collections::HashSet;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i64> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.unwrap().parse().expect("Could not parse line"))
        .collect()
}


fn main() {
    let lines = lines_from_file("input.txt");
    const PREAMBLE_SIZE: usize = 25;
    let mut invalid_number: i64 = 0;

    let mut previous_numbers: HashSet<&i64> = lines[0..PREAMBLE_SIZE].iter().collect();

    for (i, num) in lines[PREAMBLE_SIZE..].iter().enumerate() {
        let mut is_valid = false;

        for p in previous_numbers.iter() {
            let delta = *num - **p;

            if delta != **p && previous_numbers.contains(&delta) {
                is_valid = true;
                break;
            }
        }

        previous_numbers = lines[i..i+PREAMBLE_SIZE+1].iter().collect();

        if !is_valid {
            invalid_number = *num;
            break;
        }
    }

    println!("Result: {}", invalid_number);

    let mut contiguous_numbers: Vec<i64> = Vec::new();

    for n in 0..lines.len() {
        let mut i = n + 1;
        let mut sum = lines[n];
        contiguous_numbers.push(lines[n]);

        while i < lines.len() && sum < invalid_number {
            sum += lines[i];
            contiguous_numbers.push(lines[i]);

            if sum == invalid_number {
                break;
            }
            i += 1;
        }

        if sum == invalid_number {
            break;
        }

        contiguous_numbers.clear();
    }

    contiguous_numbers.sort();
    println!("Part 2 result: {}", contiguous_numbers[0] + contiguous_numbers.last().unwrap());
}
