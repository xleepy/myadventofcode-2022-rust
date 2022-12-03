use super::filereader;
use std::collections::HashMap;

fn play_game(player_shape_value: &i32, opponent_shape_value: &i32) -> i32 {
    if player_shape_value == opponent_shape_value {
        return player_shape_value + 3;
    }
    if player_shape_value > opponent_shape_value {
        return player_shape_value + 6;
    }
    if player_shape_value < opponent_shape_value {
        return player_shape_value.clone();
    }
    return 0;
}

pub fn run() {
    let lines = filereader::read_lines("./src/day2/input.txt").expect("Cannot parse file");
    let shapes_map = HashMap::from([("X", "A"), ("Y", "B"), ("Z", "C")]);
    let results_map = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let mut all_games_result = 0;
    for line in lines {
        if line.is_ok() {
            let row = line.unwrap();
            let columns: Vec<&str> = row.split(" ").collect();
            let player_shape = shapes_map
                .get(columns.last().unwrap())
                .expect("Cannot find shape");

            let player_shape_value = results_map.get(player_shape).unwrap();
            let opponent_shape_value = results_map
                .get(columns.first().unwrap())
                .expect("Canno parse value");

            let result = play_game(player_shape_value, opponent_shape_value);
            all_games_result += result;
        }
    }

    println!("day 2 {:?} ", all_games_result);
}
