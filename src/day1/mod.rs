use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

pub fn run() {
    let path = fs::canonicalize("./src/day1/1.txt").expect("File not found");
    let file = File::open(path).expect("Cannot open file");
    let lines = io::BufReader::new(file).lines();
    let mut elves: Vec<i32> = Vec::new();
    let mut current_sum = 0;
    for line in lines {
        if line.is_ok() {
            let current_line = line.unwrap();
            if current_line.is_empty() {
                elves.push(current_sum);
                current_sum = 0;
            } else {
                let parsed = current_line.parse::<i32>().unwrap();
                current_sum += parsed;
            }
        }
    }
    let mut collected: Vec<i32> = elves.clone();
    collected.sort();
    collected.truncate(3);
    let sum_of_three: i32 = collected.iter().sum();

    println!("{:?}", sum_of_three);
}
