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
    let movements = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let mut result: i64 = 1;
    let line_len = lines[0].len();

    for movement in &movements {
        let mut num_trees = 0;
        let mut current_x_pos = 0;

        for i in (movement[1]..lines.len()).step_by(movement[1]) {
            current_x_pos = (current_x_pos + movement[0]) % line_len;
            let path_object = lines[i].chars().nth(current_x_pos).unwrap();
    
            if path_object == '#' {
                num_trees += 1;
            }
        }
        println!("Number of Trees in Path: {}", num_trees);

        result *= num_trees;
    }

    println!("Result: {}", result);
}