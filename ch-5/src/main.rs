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
    let mut seats = HashSet::new();
    let mut correct_seat = 0;

    for line in lines {
        let row_string = &line[0..7].replace("B", "1").replace("F", "0");
        let row_number = isize::from_str_radix(row_string, 2).unwrap();

        let column_string = &line[7..].replace("R", "1").replace("L", "0");
        let column_number = isize::from_str_radix(column_string, 2).unwrap();

        let seat_id = row_number * 8 + column_number;

        seats.insert(seat_id);
        println!("{:?}", seat_id);
    }

    for seat in seats.iter() {
        if seats.contains(&(seat + 2)) && !seats.contains(&(seat + 1)) {
            correct_seat = *seat + 1;
            break;
        }
    }

    println!("Your seat: {:?}", correct_seat);
}