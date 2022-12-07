use super::filereader;

fn parse(value: &str) -> Vec<i32> {
    let result: Vec<i32> = value
        .split("-")
        .map(|x| x.parse::<i32>().unwrap())
        .into_iter()
        .collect();
    return result;
}

fn check_pair(first_val: &Vec<i32>, second_val: &Vec<i32>) -> bool {
    return first_val.first() >= second_val.first() && first_val.last() <= second_val.last()
        || first_val.first() <= second_val.last() && first_val.last() >= second_val.first();
}

pub fn run() {
    let lines = filereader::read_lines("./src/day4/input.txt").expect("Cannot read file");
    let mut pairs_sum = 0;

    for line in lines {
        let pair: Vec<&str> = line.split(",").collect();
        let first_elf = parse(pair.first().expect("Cannot find value"));
        let second_elf = parse(pair.last().expect("cannot find value"));
        if check_pair(&first_elf, &second_elf) || check_pair(&second_elf, &first_elf) {
            pairs_sum += 1;
        }
    }
    println!("day 4: {:?}", pairs_sum);
}
