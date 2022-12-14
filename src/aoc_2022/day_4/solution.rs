use std::io::BufRead;

use crate::util::file::get_input_reader;
use crate::aoc_2022::constants::YEAR;

const PROBLEM: &str = "day_4";

struct Range {
    floor: i32,
    ceiling: i32,
}

fn build_range(data: &str) -> Range {
    let vals: Vec<&str> = data.split("-").collect();

    Range {
        floor: vals[0].parse::<i32>().unwrap(),
        ceiling: vals[1].parse::<i32>().unwrap(),
    }
}

/// is A (in its entirety) contained in B
fn contains(range_a: &Range, range_b: &Range) -> bool {
    return range_a.floor >= range_b.floor && range_a.ceiling <= range_b.ceiling;
}

fn overlap(range_a: &Range, range_b: &Range) -> bool {
    let result = range_a.ceiling < range_b.floor || range_a.floor > range_b.ceiling;
    return !result;
}

/// Camp Cleanup
/// https://adventofcode.com/2022/day/4
pub fn solve_1(filename: &str) -> i32 {
    let reader = get_input_reader(filename, YEAR, PROBLEM);
    let mut result = 0;

    for raw_line in reader.lines() {
        match raw_line {
            Ok(line) => {
                if line == "" { continue }
                let ranges: Vec<&str> = line.split(",").collect();

                let range_a = build_range(ranges[0]);
                let range_b = build_range(ranges[1]);

                if contains(&range_a, &range_b) || contains(&range_b, &range_a) {
                    result += 1;
                }
            },
            Err(error) => panic!("{}", error),
        };
    }
    return result as i32;
}

pub fn solve_2(filename: &str) -> i32 {
    let reader = get_input_reader(filename, YEAR, PROBLEM);
    let mut result = 0;

    for raw_line in reader.lines() {
        match raw_line {
            Ok(line) => {
                if line == "" { continue }
                let ranges: Vec<&str> = line.split(",").collect();

                let range_a = build_range(ranges[0]);
                let range_b = build_range(ranges[1]);

                if overlap(&range_a, &range_b) {
                    result += 1;
                }
            },
            Err(error) => panic!("{}", error),
        };
    }
    return result as i32;
}


#[cfg(test)]
mod tests {
    use super::solve_1;
    use super::solve_2;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve_1(input_file);
        assert_eq!(result, 538);
    }

    #[test]
    fn part_2() {
        let input_file = "sample.txt";
        let result = solve_2(input_file);
        assert_eq!(result, 792);
    }
}
