use std::{io::{BufReader, Error, BufRead, Lines}, fs::File, collections::VecDeque};
use crate::{aoc_2022::constants, util::file::get_current_dir};

const PROBLEM: &str = "day_11";

const ROUND_LIMIT: i32 = 20;

struct Monkey {
    items: Vec<i32>,
    inspect_fn: Box<dyn Fn(i32) -> i32>,
    test_fn: Box<dyn Fn(i32) -> usize>,
}

impl Monkey {
    fn inspect(&self, val: i32) -> i32 {
        return (&self.inspect_fn)(val);
    }

    fn test(&self, val: i32) -> usize {
        return (&self.test_fn)(val);
    }
}

fn parse_starting_items(input: &str) -> Vec<i32> {
    return input
        .split(", ")
        .map(|elem| elem.parse::<i32>().unwrap())
        .collect();
}

fn parse_operation(input: &str) -> Box<dyn Fn(i32) -> i32> {
    let tokens: Vec<&str> = input.split(" ").collect();

    let arg_a = tokens[2];
    let arg_b = tokens[4].parse::<i32>().unwrap();
    let operator = tokens[3];

    let closure = |num: i32| num * arg_b;
    return Box::new(closure);
}

fn parse_test(iter: &Lines<BufReader<File>>) -> Box<dyn Fn(i32) -> usize> {
    let line = iter.next().unwrap().unwrap();
    let tokens: Vec<&str> = line.split(" by ").collect();

    let divisor = tokens[1].parse::<i32>().unwrap();

    // Assumption 'true' condition is always first and 'false' is always second in input
    let dest_list: Vec<usize> = vec![0; 2];

    for i in 0..2 {
        line = iter.next().unwrap().unwrap();
        tokens = line.split("throw to monkey ").collect();
        let deliver_a = tokens[1].parse::<i32>().unwrap();
        dest_list[i] = deliver_a as usize;
    }

    let closure = |num: i32| if num % divisor == 0 { dest_list[0] } else { dest_list[1] };
    return Box::new(closure);
}

fn build_monkey(iter: &Lines<BufReader<File>>) -> Monkey {
    let line = iter.next().unwrap().unwrap();
    let tokens: Vec<&str> = line.split(": ").collect();
    let items = parse_starting_items(tokens[1]);

    line = iter.next().unwrap().unwrap();
    tokens = line.split(": ").collect();
    let inspect_fn = parse_operation(tokens[1]);

}

/// Monkey in the Middle
/// https://adventofcode.com/2022/day/11
pub fn solve(filename: String) -> Result<i32, Error> {
    let current_dir = get_current_dir();
    let path = format!("{}/src/{}/{}/{}", current_dir, constants::YEAR.to_string(), PROBLEM.to_string(), filename);

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let monkeys: Vec<Monkey> = vec![];

    let mut iter = reader.lines();
    while let Some(raw_line) = iter.next() {
        match raw_line {
            Ok(line) => {
                if !line.starts_with("Monkey") { panic!("Cannot read monkey signature"); }
                let new_monkey = build_monkey(iter);
                monkeys.push(new_monkey);
            },
            Err(_error) => continue,
        };
    }

    return Ok(0);
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_11::solution::solve;

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
