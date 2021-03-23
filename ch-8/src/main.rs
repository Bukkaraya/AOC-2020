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
    let instructions = lines_from_file("input.txt");
    let num_instructions = instructions.len() as i32;
    let mut executed_instructions: HashSet<i32> = HashSet::new();
    let mut current_instruction: i32 = 0;
    let mut accumulator = 0;

    while !executed_instructions.contains(&current_instruction) && current_instruction < num_instructions {
        let tokens: Vec<&str> = instructions[current_instruction as usize].split(" ").collect();

        let operation = tokens[0];
        let value: i32 = tokens[1].parse().unwrap();

        match operation {
            "acc" => {
                accumulator += value;
            },
            "jmp" => {
                current_instruction += value - 1;
            }
            "nop" => (),
            _ => ()
        }

        executed_instructions.insert(current_instruction);
        current_instruction += 1;
    }

    println!("Result: {}", accumulator);


    for (i, inst) in instructions.iter().enumerate() {
        let modified_inst: String;

        if inst.contains("jmp") {
            modified_inst = inst.replace("jmp", "nop").to_string();
        } else {
            modified_inst = inst.replace("nop", "jmp").to_string();
        }

        let mut modified_instructions = instructions.clone();

        modified_instructions[i] = modified_inst;

        executed_instructions.clear();
        accumulator = 0;
        current_instruction = 0;


        while !executed_instructions.contains(&current_instruction) && current_instruction < num_instructions {
            let tokens: Vec<&str> = modified_instructions[current_instruction as usize].split(" ").collect();
    
            let operation = tokens[0];
            let value: i32 = tokens[1].parse().unwrap();
            executed_instructions.insert(current_instruction);
        
            match operation {
                "acc" => {
                    accumulator += value;
                },
                "jmp" => {
                    current_instruction += value - 1;
                }
                "nop" => (),
                _ => ()
            }
    
            current_instruction += 1;
        }

        if current_instruction == num_instructions {
            break;
        }
    }

    println!("Part 2 Result: {}", accumulator);

}