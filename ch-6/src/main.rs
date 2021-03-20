use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};
use std::collections::HashSet;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    let lines = lines_from_file("input.txt");
    let mut answered_questions: Vec<HashSet<char>> = Vec::new();
    let mut num_answered = 0;

    for line in lines {
        if line == "" {
            num_answered += get_num_common_elements(&answered_questions);
            answered_questions.clear();
            continue;
        }

        let current_answers: HashSet<char> = line.chars().collect();

        answered_questions.push(current_answers);
    }

    num_answered += get_num_common_elements(&answered_questions);

    println!("Result: {}", num_answered);
}


fn get_num_common_elements(elements: &Vec<HashSet<char>>) -> usize{
    let mut common_elements: HashSet<char> = elements[0].iter().cloned().collect();

    for element in &elements[1..] {
        common_elements = common_elements.intersection(element).cloned().collect();
    }

    common_elements.len()
}