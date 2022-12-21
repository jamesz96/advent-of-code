use std::{io::BufRead, cmp::Ordering};
use crate::{aoc_2022::constants::YEAR, util::file::get_input_reader};

const PROBLEM: &str = "day_13";

#[derive(PartialEq, Eq, Debug)]
pub enum Packet {
    Value(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Value(b)) => a.cmp(&vec![Packet::Value(*b)]),
            (Self::Value(a), Self::Value(b)) => a.cmp(b),
            (Self::Value(a), Self::List(b)) => vec![Packet::Value(*a)].cmp(&b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packet(line: &str) -> Packet {
    let mut stack: Vec<Packet> = Vec::new();
    let mut iter = line.chars().peekable();

    let mut root = Packet::List(vec![Packet::Value(0)]);

    while let Some(char) = iter.next() {
        match char {
            ',' => continue,
            '[' => {
                let new_packet = vec![];
                stack.push(Packet::List(new_packet));
            },
            ']' => {
                let packet = stack.pop().unwrap();
                match stack.last_mut() {
                    Some(parent) => {
                        match parent {
                            Packet::List(x) => _ = x.push(packet),
                            Packet::Value(_) => (),
                        };
                    },
                    None => root = packet,
                };
            },
            _ => {
                let parent = stack.last_mut().unwrap();
                match parent {
                    Packet::Value(_) => (),
                    Packet::List(list) => {
                        let mut digits: Vec<char> = vec![char];
                        while let Some(digit) = iter.peek() {
                            if digit.is_digit(10) { digits.push(*digit); } else { break; }
                            iter.next();
                        }
                        let number: String = digits.iter().collect();
                        let value = String::from(number).parse::<i32>().unwrap();
                        list.push(Packet::Value(value));
                    }
                }
            }
        }
    }

    return root;
}


/// Distress Signal
/// https://adventofcode.com/2022/day/13
pub fn solve(filename: &str) -> i32 {
    let reader = get_input_reader(filename, YEAR, PROBLEM);

    let mut result = 0;

    let mut idx = 1;
    let mut iter = reader.lines();

    while let Some(line) = iter.next() {
        match line {
            Ok(input) => {
                let packet_a = parse_packet(&input);
                let packet_b = parse_packet(&iter.next().unwrap().unwrap());

                if packet_a < packet_b { result += idx; }

                // Skip blank line
                iter.next();
                idx += 1;
            },
            Err(error) => panic!("{}", error),
        };
    }
    return result;
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_13::solution::solve;

    #[test]
    fn part_1_test() {
        let input_file = "test.txt";
        let result = solve(input_file);
        assert_eq!(result, 13);
    }

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file);
        assert_eq!(result, 5340);
    }
}
