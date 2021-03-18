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
            if valid_keys.len() == required_keys.len() {
                num_valid += 1;
            }

            valid_keys = Vec::new();
            continue;
        }
        let pairs = line.split(" ");
        
        for pair in pairs {
            let mapping: Vec<&str> = pair.split(":").collect();
            
            let is_valid: bool = match mapping[0] {
                "byr" => {
                    let birth_year: i32 = mapping[1].parse().unwrap_or(0);
                    birth_year >= 1920 && birth_year <= 2002 
                },
                "iyr" => {
                    let issue_year: i32 = mapping[1].parse().unwrap_or(0);
                    issue_year >= 2010 && issue_year <= 2020

                },
                "eyr" => {
                    let expiration_year: i32 = mapping[1].parse().unwrap_or(0);
                    expiration_year >= 2020 && expiration_year <= 2030
                },
                "hgt" => {
                    let height = mapping[1];

                    if height.contains("cm") {
                        let height = height.replace("cm", "");
                        let height: i32 = height.parse().unwrap_or(0);

                        height >= 150 && height <= 193
                    } else if height.contains("in") {
                        let height = height.replace("in", "");
                        let height: i32 = height.parse().unwrap_or(0);

                        height >= 59 && height <= 76
                    } else {
                        false
                    }
                },
                "hcl" => {
                    let hair_color = mapping[1];
                    let hex_chars = "abcdef0123456789";
                    let mut is_valid_hex = true;

                    if hair_color.chars().nth(0).unwrap() == '#' && hair_color.len() == 7{
                        for c in hair_color[1..].chars() {
                            if !hex_chars.contains(c) {
                                is_valid_hex = false;
                                break;
                            }
                        }

                        is_valid_hex
                    } else {
                        false
                    }
                },
                "ecl" => {
                    let allowed_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    let eye_color = mapping[1];

                    allowed_colors.contains(&eye_color)
                },
                "pid" => {
                    let passport_id = mapping[1];
                    let mut is_id_valid = true;

                    if passport_id.len() == 9 {
                        let passport_id: i64 = passport_id.parse().unwrap_or(-1);
                        if passport_id == -1 {
                            is_id_valid = false;
                        }
                    } else {
                        is_id_valid = false;
                    }

                    is_id_valid
                },
                "cid" => false,
                _ => false
            };

            if is_valid {
                valid_keys.push(mapping[0].clone());
            }
        }
    }

    if valid_keys.len() == required_keys.len() {
        num_valid += 1;
    }

    println!("Result: {:?}", num_valid);
}