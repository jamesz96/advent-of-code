use crate::{util::file::read_input, aoc_2022::constants};

const PROBLEM: &str = "day_7";

/// No Space Left On Device
/// https://adventofcode.com/2022/day/7
pub fn solve(filename: String) -> i32 {
    let _contents = read_input(filename, constants::YEAR.to_string(), PROBLEM.to_string());
    return 0;
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_7::solution::solve;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        assert_eq!(result, 1896);
    }
}
