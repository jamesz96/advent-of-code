use std::{io::{BufReader, Error, BufRead}, fs::File};
use crate::{aoc_2022::constants, util::file::get_current_dir};

const PROBLEM: &str = "day_13";

#[derive(Debug)]
enum Element {
    Value(i32),
    Packet(Packet),
}

enum CompareResult {
    Same,
    InOrder,
    OutOfOrder,
}

#[derive(Debug)]
struct Packet {
    elements: Vec<Element>,
}

fn is_in_order(a: &Packet, b: &Packet) -> CompareResult {
    let mut idx_a = 0;
    let mut idx_b = 0;

    while idx_a < a.elements.len() && idx_b < b.elements.len() {
        let element_a = &a.elements[idx_a];
        let element_b = &b.elements[idx_b];

        match element_a {
            Element::Value(val_a) => {
                match element_b {
                    Element::Value(val_b) => {
                        println!("-----");
                        println!("{}", val_a);
                        println!("{}", val_b);
                        if val_a > val_b { return CompareResult::OutOfOrder }
                        if val_a < val_b { return CompareResult::InOrder; }
                    },
                    Element::Packet(packet_b) => {
                        let transformed_packet_a = Packet { elements: vec![Element::Value(*val_a)] };
                        let result = is_in_order(&transformed_packet_a, packet_b);
                        match result {
                            CompareResult::Same => (),
                            _ => return result,
                        };
                    },
                };
            },
            Element::Packet(packet_a) => {
                match element_b {
                    Element::Value(val_b) => {
                        let transformed_packet_b = Packet { elements: vec![Element::Value(*val_b)] };
                        let result = is_in_order(packet_a, &transformed_packet_b);
                        match result {
                            CompareResult::Same => (),
                            _ => return result,
                        };
                    },
                    Element::Packet(packet_b) => {
                        let result = is_in_order(packet_a, packet_b);
                        match result {
                            CompareResult::Same => (),
                            _ => return result,
                        }
                    }
                };
            }
        };

        idx_a += 1;
        idx_b += 1;
    }

    if idx_a < a.elements.len() && idx_b == b.elements.len() {
        return CompareResult::OutOfOrder;
    }
    return CompareResult::Same;
}

fn parse_packet(line: &str) -> Packet {
    let mut root = Packet { elements: vec![] };
    let mut stack: Vec<Packet> = Vec::new();

    for char in line.chars() {
        match char {
            ',' => continue,
            '[' => {
                let new_packet = Packet { elements: vec![] };
                stack.push(new_packet);
            },
            ']' => {
                let packet = stack.pop().unwrap();
                match stack.last_mut() {
                    Some(parent) => {
                        let packet_element = Element::Packet(packet);
                        parent.elements.push(packet_element)
                    },
                    None => root = packet,
                };
            },
            _ => {
                let parent = stack.last_mut().unwrap();

                let value = String::from(char).parse::<i32>().unwrap();
                let value_element = Element::Value(value);

                parent.elements.push(value_element);
            }
        }
    }

    return root;
}


/// Distress Signal
/// https://adventofcode.com/2022/day/13
pub fn solve(filename: String) -> Result<i32, Error> {
    let current_dir = get_current_dir();
    let path = format!("{}/src/{}/{}/{}", current_dir, constants::YEAR.to_string(), PROBLEM.to_string(), filename);

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result = 0;

    let mut idx = 1;
    let mut iter = reader.lines();

    while let Some(line) = iter.next() {
        match line {
            Ok(input) => {
                let packet_a = parse_packet(&input);
                let packet_b = parse_packet(&iter.next().unwrap().unwrap());

                let order_result = is_in_order(&packet_a, &packet_b);
                match order_result {
                    CompareResult::OutOfOrder => {
                        result += idx;
                        println!("{}", false);
                    },
                    _ => println!("{}", true),
                }

                // Skip blank line
                iter.next();

                idx += 1;
            },
            Err(error) => panic!("{}", error),
        };
    }



    return Ok(result);
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_13::solution::solve;

    #[ignore]
    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        match result {
            Ok(x) => assert_eq!(x, 13),
            Err(_error) => return,
        }
    }

    // #[ignore]
    #[test]
    fn test() {
        let input_file = "test.txt";
        let result = solve(input_file.to_string());
        match result {
            Ok(x) => assert_eq!(x, 13),
            Err(_error) => return,
        }
    }
}
