use std::collections::{VecDeque, HashMap};

use crate::{util::file::read_input, aoc_2022::constants};

const PROBLEM: &str = "day_5";

const BUFFER_SIZE: usize = 4;

/// Tuning Trouble
/// https://adventofcode.com/2022/day/6
pub fn solve(filename: String) -> i32 {
    let contents = read_input(filename, constants::YEAR.to_string(), PROBLEM.to_string());
    let mut set: HashMap<char, i32> = HashMap::new();
    let mut buffer: VecDeque<char> = VecDeque::new();

    for line in contents.split("\n") {
        let mut chars = line.chars();
        while let Some(letter) = chars.next() {
            if buffer.len() == BUFFER_SIZE {
                let element = buffer.pop_front();
                match element { Some(x) => set.remove(&x), None => continue };
            }

        }
    }
    return 0;
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_6::solution::solve;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        assert_eq!(result, 0);
    }
}
