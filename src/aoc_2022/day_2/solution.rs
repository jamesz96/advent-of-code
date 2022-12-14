use std::collections::HashMap;
use std::io::BufRead;

use crate::util::file::get_input_reader;
use crate::aoc_2022::constants::YEAR;

const PROBLEM: &str = "day_2";

const ROCK: usize = 0;
const PAPER: usize = 1;
const SCISSOR: usize = 2;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayResult {
    LOSS,
    TIE,
    WIN,
}

pub fn transform(letter: &str) -> usize {
    match letter {
        "A" | "X" => return ROCK,
        "B" | "Y" => return PAPER,
        "C" | "Z" => return SCISSOR,
        _ => return 0
    }
}

pub fn transform_part_2(letter: &str) -> PlayResult {
    match letter {
        "X" => return PlayResult::LOSS,
        "Y" => return PlayResult::TIE,
        "Z" => return PlayResult::WIN,
        _ => panic!("UNKNOWN character"),
    }
}

/// Rock Paper Scissors
/// https://adventofcode.com/2022/day/2
pub fn solve(filename: &str) -> i32 {
    let reader = get_input_reader(filename, YEAR, PROBLEM);

    let mut result_table = [[PlayResult::TIE; 3]; 3];

    result_table[ROCK][SCISSOR] = PlayResult::LOSS;
    result_table[ROCK][ROCK] = PlayResult::TIE;
    result_table[ROCK][PAPER] = PlayResult::WIN;

    result_table[PAPER][ROCK] = PlayResult::LOSS;
    result_table[PAPER][PAPER] = PlayResult::TIE;
    result_table[PAPER][SCISSOR] = PlayResult::WIN;

    result_table[SCISSOR][PAPER] = PlayResult::LOSS;
    result_table[SCISSOR][SCISSOR] = PlayResult::TIE;
    result_table[SCISSOR][ROCK] = PlayResult::WIN;

    let mut score = 0;

    for raw_line in reader.lines() {
        match raw_line {
            Ok(line) => {
                if line == "" { continue; }
                let values: Vec<&str> = line.split(" ").collect();
                let opp_action = transform(values[0]);
                let my_action = transform(values[1]);

                let mut round_score = (my_action as i32) + 1;

                let result = result_table[opp_action][my_action];
                if result == PlayResult::LOSS {
                    round_score += 0;
                } else if result == PlayResult::TIE {
                    round_score += 3;
                } else if result == PlayResult::WIN {
                    round_score += 6;
                }
                score += round_score;
            },
            Err(error) => panic!("{}", error)
        };
    }
    return score;
}

fn make_map(entries: &[usize; 3]) -> HashMap<PlayResult, usize> {
    let mut result: HashMap<PlayResult, usize> = HashMap::new();
    result.insert(PlayResult::WIN, entries[0]);
    result.insert(PlayResult::TIE, entries[1]);
    result.insert(PlayResult::LOSS, entries[2]);
    return result;
}

pub fn solve_part_2(filename: &str) -> i32 {
    let mut table: HashMap<usize, HashMap<PlayResult, usize>> = HashMap::new();

    let rock_entries: HashMap<PlayResult, usize> = make_map(&[PAPER, ROCK, SCISSOR]);
    let paper_entries: HashMap<PlayResult, usize> = make_map(&[SCISSOR, PAPER, ROCK]);
    let scissor_entries: HashMap<PlayResult, usize> = make_map(&[ROCK, SCISSOR, PAPER]);

    table.insert(ROCK, rock_entries);
    table.insert(PAPER, paper_entries);
    table.insert(SCISSOR, scissor_entries);

    let reader = get_input_reader(filename, YEAR, PROBLEM);

    let mut score = 0;

    for raw_line in reader.lines() {
        match raw_line {
            Ok(line) => {
                if line == "" { continue; }
                let values: Vec<&str> = line.split(" ").collect();
                let opp_action = transform(values[0]);
                let play_result = transform_part_2(values[1]);

                let my_action = table
                    .get(&opp_action).unwrap()
                    .get(&play_result).unwrap();

                let mut round_score = (*my_action as i32) + 1;

                if play_result == PlayResult::LOSS {
                    round_score += 0;
                } else if play_result == PlayResult::TIE {
                    round_score += 3;
                } else if play_result == PlayResult::WIN {
                    round_score += 6;
                }
                score += round_score;
            },
            Err(error) => panic!("{}", error)
        }
    }

    return score;
}


#[cfg(test)]
mod tests {
    use super::solve;
    use super::solve_part_2;

    #[test]
    fn test() {
        let input_file = "sample.txt";
        let result = solve(input_file);
        assert_eq!(result, 13565);
    }

    #[test]
    fn part_2() {
        let input_file = "sample.txt";
        let result = solve_part_2(input_file);
        assert_eq!(result, 12424);
    }
}
