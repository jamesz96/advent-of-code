use std::{io::{BufReader, Error, BufRead}, fs::File};
use crate::{aoc_2022::constants, util::file::get_current_dir};

const PROBLEM: &str = "day_10";

const INTERVAL_LENGTH: i32 = 40;
const START_CYCLE: i32 = 20;
const MEASUREMENT_LIMIT: i32 = 6;

/// Cathode-Ray Tube
/// https://adventofcode.com/2022/day/10
pub fn solve(filename: String) -> Result<i32, Error> {
    let current_dir = get_current_dir();
    let path = format!("{}/src/{}/{}/{}", current_dir, constants::YEAR.to_string(), PROBLEM.to_string(), filename);

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut tick = 1;
    let mut register = 1;
    let mut result = 0;
    let mut number_of_measurements = 0;

    let mut iter = reader.lines();
    while let Some(line) = iter.next() {
        match line {
            Ok(command) => {
                // println!("{}", command);
                if command == "" { continue; }
                if command  == "noop" {
                    if tick >= START_CYCLE && (tick - START_CYCLE) % INTERVAL_LENGTH == 0 {
                        println!("{}, {}", tick, register);
                        result += tick * register;
                        number_of_measurements += 1;
                    }
                    tick += 1;
                } else {
                    let splits: Vec<&str> = command.split(" ").collect();
                    let val = splits[1].parse::<i32>().unwrap();

                    for _ in 0..2 {
                        if tick >= START_CYCLE && (tick - START_CYCLE) % INTERVAL_LENGTH == 0 {
                            println!("{}, {}", tick, register);
                            result += tick * register;
                            number_of_measurements += 1;
                        }
                        tick += 1;
                    }
                    register += val;
                }
                if number_of_measurements == MEASUREMENT_LIMIT { break; }
            },
            Err(_error) => continue,
        }
    }
    println!("{}", tick);
    return Ok(result);
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_10::solution::solve;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        match result {
            Ok(x) => assert_eq!(x, 14540),
            Err(_error) => return,
        }
    }
}
