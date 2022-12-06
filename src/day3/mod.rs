use super::filereader;

pub fn run() {
    let lines = filereader::read_lines("./src/day3/test.txt").expect("Cannot parse file");
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;
    for line in lines {
        if line.is_ok() {
            let row = line.unwrap();
            let len = row.len() / 2;
            let (first, second) = row.split_at(len);
            let first_chunk: Vec<&str> = first.split("").collect();
            let second_chunk: Vec<&str> = second.split("").collect();
            let contain_in_both = first_chunk
                .into_iter()
                .filter(|x| !x.is_empty())
                .find(|x| second_chunk.contains(x))
                .expect("Cannot find char");
            let priority = chars.find(contain_in_both).expect("cannot find priority");
            sum += priority + 1;
        }
    }
    println!("day 3: {:?}", sum);
}
