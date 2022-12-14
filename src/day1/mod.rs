use super::filereader;
use std::vec::Vec;

pub fn run() {
    let lines = filereader::read_lines("./src/day1/1.txt").expect("Error while file parsing");
    let mut elves: Vec<i32> = Vec::new();
    let mut current_sum = 0;
    for line in lines {
        if line.is_empty() {
            elves.push(current_sum);
            current_sum = 0;
        } else {
            let parsed = line.parse::<i32>().unwrap();
            current_sum += parsed;
        }
    }
    let mut collected: Vec<i32> = elves.clone();
    collected.sort_by(|x, y| y.cmp(x));
    collected.truncate(3);
    let sum_of_three: i32 = collected.iter().sum();

    println!("day1: {:?}", sum_of_three);
}
