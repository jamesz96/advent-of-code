use std::collections::VecDeque;

use crate::util::file::read_input;
use crate::aoc_2022::constants;

const PROBLEM: &str = "day_5";

const TOKEN_SIZE: usize = 3;

fn parse_configuration(raw_config: &[&str], config: &mut Vec<VecDeque<char>>) {
    for line in raw_config {
        let mut chars = line.chars();
        let mut col_idx = 0;
        while let Some(c) = chars.next() {
            let mut letter = c;
            for i in 0..TOKEN_SIZE+1 {
                if i == 1 {
                    if letter != ' ' {
                        config[col_idx].push_front(letter);
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

fn rearrange(instruction: &Instruction, config: &mut Vec<VecDeque<char>>) {
    let source_idx = (instruction.source as usize) - 1;
    let destination_idx = (instruction.destination as usize) - 1;

    for _i in 0..instruction.quantity {
        let item = config[source_idx].pop_back();
        match item {
            Some(x) => config[destination_idx].push_back(x),
            None => continue,
        }
    }
}

/// Supply Stacks
/// https://adventofcode.com/2022/day/5
pub fn solve(filename: String) -> String {
    let contents = read_input(filename, constants::YEAR.to_string(), PROBLEM.to_string());
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut idx = 0;
    let mut num_columns = 0;

    while !(lines[idx] == "") {
        if lines[idx].starts_with(" 1") {
            let columns: Vec<&str> = lines[idx].split(" ").collect();
            num_columns = columns[columns.len()-1].parse::<usize>().unwrap();
        }
        idx += 1;
    }

    let mut configuration: Vec<VecDeque<char>> = vec![VecDeque::new(); num_columns];

    parse_configuration(&lines[..idx-1], &mut configuration);

    while idx < lines.len() {
        if lines[idx] == "" {
            idx += 1;
            continue;
        }
        let instruction = parse_instruction(lines[idx]);
        rearrange(&instruction, &mut configuration);
        idx += 1;
    }


    let mut top_items: Vec<char> = vec![];
    for i in 0..num_columns {
        let item = configuration[i].back();
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
    use crate::aoc_2022::day_5::solution::solve;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        assert_eq!(result, "PSNRGBTFT");
    }
}
