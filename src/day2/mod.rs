use super::filereader;
use std::collections::HashMap;

// A -> Rock 1, B -> Paper 2, C -> Scissors 3

// Rock > Scissors
// Paper > Rock
// Scissors > Paper

// pt2
// X lose
// Y draw
// Z win

// X should lose
// A > C
// B > A
// C > B

// Y draw
// A === A
// B === B
// C === C

// Z Win
// A < B
// B < C
// C < A

fn find_combo(combo: Vec<(i32, i32)>, predict: i32) -> (i32, i32) {
    return combo
        .clone()
        .into_iter()
        .find(|x| x.0 == predict)
        .expect("Cannot find combo");
}

pub fn run() {
    let lines = filereader::read_lines("./src/day2/input.txt").expect("Cannot parse file");
    let results_map = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let lose_combos = Vec::from([(1, 3), (2, 1), (3, 2)]);
    let win_combos = Vec::from([(1, 2), (2, 3), (3, 1)]);

    let mut all_games_result = 0;
    for line in lines {
        if line.is_ok() {
            let row = line.unwrap();
            let columns: Vec<&str> = row.split(" ").collect();
            let game_strategy = columns.get(1).expect("cannot find strategy").trim();

            let opponent_value = results_map
                .get(columns.first().unwrap())
                .expect("Cannot get value");
            let mut result = 0;
            if game_strategy == "Y" {
                result += opponent_value + 3;
            } else if game_strategy == "X" {
                let combo = find_combo(lose_combos.clone(), *opponent_value);
                result += combo.1;
            } else {
                let combo = find_combo(win_combos.clone(), *opponent_value);
                result += combo.1 + 6
            }

            all_games_result += result;
        }
    }

    println!("day 2 {:?} ", all_games_result);
}
