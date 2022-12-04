use std::cmp;

use crate::util::file::read_input;
use crate::aoc_2022::constants;

const PROBLEM: &str = "p_1_counting_calories";

/// 1. Calorie Counting
/// https://adventofcode.com/2022/day/1
pub fn solve(filename: String) -> i32 {
    let contents = read_input(filename, constants::YEAR.to_string(), PROBLEM.to_string());
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut idx = 0;
    let mut result = 0;

    let num_lines = lines.len();

    while idx < num_lines {
        let mut sum = 0;

        while idx < num_lines && lines[idx] != "" {
            let val = lines[idx].to_string().parse::<i32>().unwrap();
            sum += val;
            idx += 1;
        }

        result = cmp::max(result, sum);
        idx += 1;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::aoc_2022::p_1_counting_calories::solution::solve;

    #[test]
    fn it_works() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        assert_eq!(result, 70369);
    }
}
