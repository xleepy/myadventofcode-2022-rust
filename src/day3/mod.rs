use super::filereader;

pub fn run() {
    let lines = filereader::read_lines("./src/day3/input.txt").expect("Cannot parse file");
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;

    let chunks = lines.chunks(3);

    for chunk in chunks {
        println!("chunk {:?} ", chunk);
        let first = chunk
            .first()
            .clone()
            .expect("cannot find value")
            .split("")
            .filter(|x| !x.is_empty());

        let mid = chunk.get(1).expect("cannot find value");
        let last = chunk.last().expect("cannot find value");

        let value = first
            .clone()
            .find(|x| mid.contains(x) && last.contains(x))
            .expect("cannot find value");
        let position = chars.find(value).expect("cannot find char position");
        sum += position + 1;
    }

    println!("day 3: {:?}", sum);
}
