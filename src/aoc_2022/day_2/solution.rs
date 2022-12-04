use crate::util::file::read_input;
use crate::aoc_2022::constants;

const PROBLEM: &str = "day_2";

const ROCK: usize = 0;
const PAPER: usize = 1;
const SCISSOR: usize = 2;

#[derive(Clone, Copy, PartialEq)]
enum Result {
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

/// Rock Paper Scissors
/// https://adventofcode.com/2022/day/2
pub fn solve(filename: String) -> i32 {
    let contents = read_input(filename, constants::YEAR.to_string(), PROBLEM.to_string());
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut result_table = [[Result::TIE; 3]; 3];

    result_table[ROCK][SCISSOR] = Result::LOSS;
    result_table[ROCK][ROCK] = Result::TIE;
    result_table[ROCK][PAPER] = Result::WIN;

    result_table[PAPER][ROCK] = Result::LOSS;
    result_table[PAPER][PAPER] = Result::TIE;
    result_table[PAPER][SCISSOR] = Result::WIN;

    result_table[SCISSOR][PAPER] = Result::LOSS;
    result_table[SCISSOR][SCISSOR] = Result::TIE;
    result_table[SCISSOR][ROCK] = Result::WIN;

    let mut score = 0;

    for line in lines {
        if line == "" { continue; }
        let values: Vec<&str> = line.split(" ").collect();
        let opp_action = transform(values[0]);
        let my_action = transform(values[1]);

        let mut round_score = (my_action as i32) + 1;

        let result = result_table[opp_action][my_action];
        if result == Result::LOSS {
            round_score += 0;
        } else if result == Result::TIE {
            round_score += 3;
        } else if result == Result::WIN {
            round_score += 6;
        }
        score += round_score;
    }

    return score;
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_2::solution::solve;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        assert_eq!(result, 13565);
    }
}
