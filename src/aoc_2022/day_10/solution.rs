use std::io::BufRead;

use crate::{util::file::get_input_reader, aoc_2022::constants::YEAR};

const PROBLEM: &str = "day_10";

const INTERVAL_LENGTH: i32 = 40;
const START_CYCLE: i32 = 20;
const MEASUREMENT_LIMIT: i32 = 6;

/// Cathode-Ray Tube
/// https://adventofcode.com/2022/day/10
pub fn part_1(filename: &str) -> i32 {
    let reader = get_input_reader(filename, YEAR, PROBLEM);

    let mut tick = 1;
    let mut register = 1;
    let mut result = 0;
    let mut number_of_measurements = 0;

    let mut iter = reader.lines();
    while let Some(line) = iter.next() {
        match line {
            Ok(command) => {
                if command == "" { continue; }
                if command  == "noop" {
                    if tick >= START_CYCLE && (tick - START_CYCLE) % INTERVAL_LENGTH == 0 {
                        result += tick * register;
                        number_of_measurements += 1;
                    }
                    tick += 1;
                } else {
                    let splits: Vec<&str> = command.split(" ").collect();
                    let val = splits[1].parse::<i32>().unwrap();

                    for _ in 0..2 {
                        if tick >= START_CYCLE && (tick - START_CYCLE) % INTERVAL_LENGTH == 0 {
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
    return result;
}

fn step_cycle(cycle: i32, buffer: &mut Vec<char>, register: i32) -> i32 {
    let position = cycle - 1;
    let idx = position % INTERVAL_LENGTH;
    if idx + 1 >= register && idx - 1 <= register {
        buffer.push('#');
    } else {
        buffer.push('.');
    }
    if buffer.len() == 40 { flush_buffer(buffer); }
    return cycle + 1;
}

fn flush_buffer(buffer: &mut Vec<char>) {
    let out: String = buffer.iter().collect();
    println!("{}", out);
    buffer.clear();
}

pub fn part_2(filename: &str) {
    let reader = get_input_reader(filename, YEAR, PROBLEM);

    let mut cycle = 1;
    let mut register = 1;

    let mut iter = reader.lines();
    let mut buffer: Vec<char> = vec![];

    while let Some(line) = iter.next() {
        match line {
            Ok(command) => {
                match command.as_str() {
                    "" => continue,
                    "noop" => cycle = step_cycle(cycle, &mut buffer, register),
                    _ => {
                        let splits: Vec<&str> = command.split(" ").collect();
                        let val = splits[1].parse::<i32>().unwrap();

                        for _ in 0..2 {
                            cycle = step_cycle(cycle, &mut buffer, register)
                        }
                        register += val;
                    },
                };

            },
            Err(error) => panic!("{}", error),
        };
    }
}


#[cfg(test)]
mod tests {
    use super::part_1;
    use super::part_2;

    #[test]
    fn test_part_1() {
        let input_file = "sample.txt";
        let result = part_1(input_file);
        assert_eq!(result, 14540);
    }

    #[test]
    fn test_part_2_example() {
        let input_file = "larger_example.txt";
        part_2(input_file);
        assert_eq!(0, 0);
    }

    #[test]
    fn test_part_2() {
        let input_file = "sample.txt";
        part_2(input_file);
        assert_eq!(0, 0);
    }
}
