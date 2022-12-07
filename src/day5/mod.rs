use super::filereader;
use std::io::Read;

pub fn run() {
    let mut data = String::new();
    let mut file = filereader::read("./src/day5/test.txt").expect("Cannot find file");
    file.read_to_string(&mut data).expect("Cannot read string");

    println!("data {:?}", data.split("\n"));
}
