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
    let lines = lines_from_file("input.txt");
    let mut num_valid_passwords = 0;

    for line in lines {
        let tokens: Vec<&str> = line.split(" ").collect();
        let range: Vec<usize> = tokens[0].split("-")
            .map(|n| n.parse().expect("Could not parse number"))
            .collect();
        let character: char = tokens[1].replace(":", "").chars().nth(0).unwrap();
        let word = tokens[2];
        // let mut occurences = 0;
        // let sub_word = &word[(range[0] - 1)..range[1]];

        let first_pos = word.chars().nth(range[0] - 1).unwrap();
        let second_pos = word.chars().nth(range[1] - 1).unwrap();

        if (first_pos == character) ^ (second_pos == character) {
            num_valid_passwords += 1;
        }

        // println!("{:?}, {:?}, {:?}, {:?}, {:?}", range, character, sub_word, word, num_valid_passwords);
    }

    println!("Number of Valid Passwords: {}", num_valid_passwords);
}