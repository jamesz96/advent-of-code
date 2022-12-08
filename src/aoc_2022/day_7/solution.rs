use std::{io::{BufReader, Error, BufRead, Lines}, fs::File, collections::HashMap};
use crate::{aoc_2022::constants, util::file::get_current_dir};

const PROBLEM: &str = "day_7";

/// No Space Left On Device
/// https://adventofcode.com/2022/day/7
pub fn solve(filename: String) -> Result<i32, Error> {
    let current_dir = get_current_dir();
    let path = format!("{}/src/{}/{}/{}", current_dir, constants::YEAR.to_string(), PROBLEM.to_string(), filename);

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result_directories: HashMap<&str, i32> = HashMap::new();

    // for line in reader.lines() {
    //     match (line) {
    //         Ok(x) =>
    //     }
    //
    // }

    return Ok(0);
}


#[cfg(test)]
mod tests {
    // use crate::aoc_2022::day_7::solution::solve;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        assert_eq!(1, 0);
    }
}
