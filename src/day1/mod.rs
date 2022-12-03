// use std::fs;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};

pub fn run() {
    let path = fs::canonicalize("./src/day1/1.txt").expect("File not found");
    let file = File::open(path).expect("Cannot open file");
    let lines = io::BufReader::new(file).lines();
    let mut elves: HashMap<i32, i32> = HashMap::new();
    let mut current_sum = 0;
    let mut current_elf: i32 = 0;
    for line in lines {
        if line.is_ok() {
            let current_line = line.unwrap();
            if current_line.is_empty() {
                elves.insert(current_elf, current_sum);
                current_elf += 1;
                current_sum = 0;
            } else {
                let parsed = current_line.parse::<i32>().unwrap();
                current_sum += parsed;
            }
        }
    }
    let mut collected: Vec<(i32, i32)> = elves.into_iter().collect();
    collected.sort_by(|x, y| y.1.cmp(&x.1));
    collected.truncate(3);
    let sum_of_three: i32 = collected.iter().map(|v| v.1).sum();

    println!("{:?}", sum_of_three);
}
