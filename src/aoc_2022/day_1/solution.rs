use std::cmp;

use crate::util::array::fixed_sorted_array_insert;
use crate::util::file::read_input;
use crate::aoc_2022::constants;

const PROBLEM: &str = "day_1";

/// Calorie Counting - Part 1
/// https://adventofcode.com/2022/day/1
pub fn solve_part_1(filename: String) -> i32 {
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

/// Calorie Counting - Part 2
/// https://adventofcode.com/2022/day/1
pub fn solve_part_2(filename: String) -> i32 {
    let contents = read_input(filename, constants::YEAR.to_string(), PROBLEM.to_string());
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut top3 = vec![0; 3];
    let mut idx = 0;
    let num_lines = lines.len();

    while idx < num_lines {
        let mut sum = 0;

        while idx < num_lines && lines[idx] != "" {
            let val = lines[idx].to_string().parse::<i32>().unwrap();
            sum += val;
            idx += 1;
        }

        top3 = fixed_sorted_array_insert(&top3, sum);
        idx += 1;
    }
    println!("{:?}", top3);

    return top3.iter().sum();
}



#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_1::solution::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve_part_1(input_file.to_string());
        assert_eq!(result, 70369);
    }

    #[test]
    fn part_2() {
        let input_file = "sample.txt";
        let result = solve_part_2(input_file.to_string());
        assert_eq!(result, 203002);
    }
}
