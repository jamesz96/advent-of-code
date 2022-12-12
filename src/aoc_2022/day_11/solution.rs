use std::{io::{BufReader, Error, BufRead, Lines}, fs::File, collections::VecDeque};
use crate::{aoc_2022::constants, util::file::get_current_dir};

const PROBLEM: &str = "day_11";

const ROUND_LIMIT: i32 = 20;

struct Monkey {
    inspect_count: i32,
    inspect_fn: Box<dyn Fn(i32) -> i32>,
    test_fn: Box<dyn Fn(i32) -> usize>,
}

impl Monkey {
    fn inspect(&mut self, val: i32) -> i32 {
        self.inspect_count += 1;
        return (&self.inspect_fn)(val);
    }

    fn test(&self, val: i32) -> usize {
        return (&self.test_fn)(val);
    }
}

fn parse_starting_items(input: &str) -> VecDeque<i32> {
    let items: Vec<i32> = input
        .split(", ")
        .map(|elem| elem.parse::<i32>().unwrap())
        .collect();
    return VecDeque::from_iter(items);
}

fn parse_operation(input: &str) -> Box<dyn Fn(i32) -> i32> {
    let tokens: Vec<String> = input.split(" ").map(String::from).collect();

    let closure = move |num: i32| -> i32 {
        match tokens[4].as_str() {
            "old" => {
                match tokens[3].as_str() {
                    "+" => num + num,
                    "-" => num - num, // needed?
                    "*" => num * num,
                    "/" => num / num, // needed?
                    _ => panic!("Invalid operation"),
                }
            },
            _ => {
                let arg_b = tokens[4].parse::<i32>().unwrap();
                match tokens[3].as_str() {
                    "+" => num + arg_b,
                    "-" => num - arg_b, // needed?
                    "*" => num * arg_b,
                    "/" => num / arg_b, // needed?
                    _ => panic!("Invalid operation"),
                }
            }
        }
    };

    return Box::new(closure);
}

fn get_test_fn(iter: &mut Lines<BufReader<File>>) -> Box<dyn Fn(i32) -> usize> {
    let line = iter.next().unwrap().unwrap();
    let tokens: Vec<&str> = line.split(" by ").collect();

    let divisor = tokens[1].parse::<i32>().unwrap();

    // Assumption 'true' condition is always first and 'false' is always second in input
    let mut dest_list: Vec<usize> = vec![0; 2];

    for i in 0..2 {
        let deliver_line = iter.next().unwrap().unwrap();
        let deliver_tokens: Vec<&str> = deliver_line.split("throw to monkey ").collect();
        let deliver_a = deliver_tokens[1].parse::<i32>().unwrap();
        dest_list[i] = deliver_a as usize;
    }

    let closure = move |num: i32| if num % divisor == 0 { dest_list[0] } else { dest_list[1] };
    return Box::new(closure);
}

fn get_starting_items(iter: &mut Lines<BufReader<File>>) -> VecDeque<i32> {
    let line = iter.next().unwrap().unwrap();
    let tokens: Vec<&str> = line.split(": ").collect();
    return parse_starting_items(tokens[1]);
}

fn get_operation_fn(iter: &mut Lines<BufReader<File>>) -> Box<dyn Fn(i32) -> i32> {
    let line = iter.next().unwrap().unwrap();
    let tokens: Vec<&str> = line.split(": ").collect();
    return parse_operation(tokens[1]);
}

fn build_monkey(iter: &mut Lines<BufReader<File>>) -> Monkey {
    let inspect_fn = get_operation_fn(iter);
    let test_fn = get_test_fn(iter);

    Monkey { inspect_count: 0, inspect_fn, test_fn }
}

/// Monkey in the Middle
/// https://adventofcode.com/2022/day/11
pub fn solve(filename: String) -> Result<i32, Error> {
    let current_dir = get_current_dir();
    let path = format!("{}/src/{}/{}/{}", current_dir, constants::YEAR.to_string(), PROBLEM.to_string(), filename);

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut arena: Vec<VecDeque<i32>> = vec![];
    let mut monkeys: Vec<Monkey> = vec![];

    let mut iter = reader.lines();
    while let Some(raw_line) = iter.next() {
        match raw_line {
            Ok(line) => {
                if line == "" { continue; }
                if !line.starts_with("Monkey") { panic!("Cannot read monkey signature"); }
                arena.push(get_starting_items(&mut iter));
                let new_monkey = build_monkey(&mut iter);
                monkeys.push(new_monkey);
            },
            Err(_error) => continue,
        };
    }

    let num_monkeys = monkeys.len();

    for _ in 0..ROUND_LIMIT {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            let items = &mut arena[idx];
            let mut buffer: Vec<VecDeque<i32>> = vec![VecDeque::new(); num_monkeys];

            while let Some(value) = items.pop_front() {
                let new_value = monkey.inspect(value) / 3;
                let target_monkey_idx = monkey.test(new_value);
                buffer[target_monkey_idx].push_back(new_value);
            }

            for i in 0..num_monkeys {
                let items = &mut arena[i];
                while let Some(item) = buffer[i].pop_front() {
                    items.push_back(item);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));

    let result = monkeys[0].inspect_count * monkeys[1].inspect_count;
    return Ok(result);
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_11::solution::solve;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        match result {
            Ok(x) => assert_eq!(x, 67830),
            Err(_error) => return,
        }
    }
}
