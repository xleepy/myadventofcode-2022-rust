use std::fs;
use std::fs::File;
use std::io::Result;
use std::io::{self, BufRead, BufReader};

pub fn read(path: &str) -> Result<File> {
    let path = fs::canonicalize(path).expect("File not found");
    let file = File::open(path).expect("Cannot open file");
    Ok(file)
}

pub fn read_lines(path: &str) -> Result<std::io::Lines<BufReader<File>>> {
    let file = read(path).expect("Issues with opening file");
    let lines = io::BufReader::new(file).lines();
    Ok(lines)
}
