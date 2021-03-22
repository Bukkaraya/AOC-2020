use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};
use std::collections::HashMap;
use std::collections::VecDeque;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    let lines = lines_from_file("input.txt");
    let mut bags: HashMap<String, HashMap<String, i32>>  = HashMap::new();

    for line in lines {
        let tokens: Vec<String> = line.split("contain")
            .map(|t| t.trim().replace("bags", "").replace("bag", "").replace(".", ""))
            .collect();
        
            let bag_name = tokens[0].trim().to_string();

            if tokens[1].trim() == "no other" {
                continue
            }

            let included_bags: Vec<&str> = tokens[1].split(",").map(|b| b.trim()).collect();

            let mut bag_details: HashMap<String, i32>  = HashMap::new();
            for bag in included_bags {
                let details: Vec<&str> = bag.split(" ").collect();
                let current_bag_name = bag.replace(details[0], "").trim().to_string();

                bag_details.insert(current_bag_name, 
                    details[0].parse().unwrap());
            }
            
            bags.insert(bag_name, bag_details);
    }

    let mut num_bags_with_shiny = 0;

    for (_bag, nested_bags) in &bags {
        let mut bags_to_check: VecDeque<&String> = nested_bags.keys().collect();

        while !bags_to_check.is_empty() {
            let current_bag = bags_to_check.pop_front().unwrap();
            if current_bag == "shiny gold" {
                num_bags_with_shiny += 1;
                break;
            }

            match bags.get(current_bag) {
                Some(next_set) => {
                    for k in next_set.keys() {
                        bags_to_check.push_back(k);
                    }
                    ()
                },
                None => {}
            };
        }
    }

    println!("Part 1 Result: {}", num_bags_with_shiny);
    let mut num_bags_in_shiny = 0;


    for (bag, count) in bags.get("shiny gold").unwrap() {
        num_bags_in_shiny += get_total_num_bags(&bags, bag, count);
    }
    println!("Result: {}", num_bags_in_shiny);
}

fn get_total_num_bags(bags: &HashMap<String, HashMap<String, i32>>, bag_name: &String, count: &i32) -> i32 {
    return match bags.get(bag_name) {
        Some(next_set) => {
            let mut sum = 0;
            for (key, value) in next_set {
                sum += count * get_total_num_bags(bags, key, value);
            }
            return sum + count;
        },
        None => count * 1
    }
}