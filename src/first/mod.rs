// use std::fs;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};

pub fn run() {
    let path = fs::canonicalize("./src/first/1.txt").expect("File not found");
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

    let max = elves.iter().max_by(|x, y| x.1.cmp(y.1));
    print!("{:?}", max);
}
