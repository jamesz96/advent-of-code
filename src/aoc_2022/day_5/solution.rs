use std::collections::VecDeque;
use std::io::BufRead;

use crate::util::file::get_input_reader;
use crate::aoc_2022::constants::YEAR;

const PROBLEM: &str = "day_5";
const TOKEN_SIZE: usize = 3;

#[derive(PartialEq)]
pub enum CraneType {
    Normal,
    CrateMover9001,
}

struct Instruction {
    quantity: i32,
    source: i32,
    destination: i32,
}

fn parse_instruction(raw_instruction: &str) -> Instruction {
    let tokens: Vec<&str> = raw_instruction.split(" ").collect();

    Instruction {
        quantity: tokens[2].parse::<i32>().unwrap(),
        source: tokens[4].parse::<i32>().unwrap(),
        destination: tokens[6].parse::<i32>().unwrap(),
    }
}

fn parse_configuration(raw_config: &Vec<String>, num_columns: usize) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![Vec::new(); num_columns];
    for line in raw_config.iter() {
        let mut chars = line.chars();
        let mut col_idx = 0;
        while let Some(c) = chars.next() {
            let mut letter = c;
            for i in 0..TOKEN_SIZE+1 {
                if i == 1 {
                    if letter != ' ' {
                        result[col_idx].push(letter);
                    }
                } else {
                    let result = chars.next();
                    match result {
                        Some(x) => letter = x,
                        None => break,
                    }
                }
            }
            col_idx += 1;
        }
    }
    for i in 0..num_columns { result[i].reverse(); }
    return result;
}


fn rearrange(instruction: &Instruction, config: &mut Vec<Vec<char>>) {
    let source_idx = (instruction.source as usize) - 1;
    let destination_idx = (instruction.destination as usize) - 1;

    for _i in 0..instruction.quantity {
        let item = config[source_idx].pop();
        match item {
            Some(x) => config[destination_idx].push(x),
            None => continue,
        }
    }
}

fn rearrange_ext(instruction: &Instruction, config: &mut Vec<Vec<char>>) {
    let source_idx = (instruction.source as usize) - 1;
    let destination_idx = (instruction.destination as usize) - 1;

    let mut ordered_buffer: VecDeque<char> = VecDeque::new();

    for _i in 0..instruction.quantity {
        let item = config[source_idx].pop();
        match item {
            Some(x) => ordered_buffer.push_front(x),
            None => continue,
        }
    }

    while ordered_buffer.len() > 0 {
        let item = ordered_buffer.pop_front();
        match item {
            Some(x) => config[destination_idx].push(x),
            None => continue,
        }
    }
}

/// Supply Stacks
/// https://adventofcode.com/2022/day/5
pub fn solve(filename: &str, crane_type: CraneType) -> String {
    let reader = get_input_reader(filename, YEAR, PROBLEM);
    let mut num_columns = 0;

    let mut iter = reader.lines();
    let mut raw_config: Vec<String> = vec![];
    while let Some(raw_line) = iter.next() {
        match raw_line {
            Ok(line) => {
                if line.starts_with(" 1") {
                    let columns: Vec<&str> = line.split(" ").collect();
                    num_columns = columns[columns.len()-1].parse::<usize>().unwrap();
                    break;
                } else {
                    raw_config.push(line);
                }
            },
            Err(error) => panic!("{}", error)
        };
    }

    let mut configuration: Vec<Vec<char>> = parse_configuration(&raw_config, num_columns);

    while let Some(raw_line) = iter.next() {
        match raw_line {
            Ok(line) => {
                if line == "" { continue; }
                let instruction = parse_instruction(&line);
                if crane_type == CraneType::Normal {
                    rearrange(&instruction, &mut configuration);
                } else if crane_type == CraneType::CrateMover9001 {
                    rearrange_ext(&instruction, &mut configuration)
                }
            },
            Err(error) => panic!("{}", error),
        };
    }


    let mut top_items: Vec<char> = vec![];
    for i in 0..num_columns {
        let item = configuration[i].last();
        match item {
            Some(x) => top_items.push(*x),
            None => continue,
        }
    }

    let result: String = top_items.into_iter().collect();
    return result;
}


#[cfg(test)]
mod tests {
    use super::{solve, CraneType};

    #[ignore]
    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file, CraneType::Normal);
        assert_eq!(result, "PSNRGBTFT");
    }

    #[test]
    fn part_2() {
        let input_file = "sample.txt";
        let result = solve(input_file, CraneType::CrateMover9001);
        assert_eq!(result, "BNTZFPMMW");
    }
}
