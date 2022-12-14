use std::collections::HashSet;
use std::io::BufRead;

use crate::aoc_2022::constants::YEAR;
use crate::util::char::get_letter_idx;
use crate::util::file::get_input_reader;

const PROBLEM: &str = "day_3";
const ALPHABET_LENGTH: u32 = 26;

fn get_priority_value(letter: char) -> u32 {
    if letter.is_ascii_lowercase() {
        return get_letter_idx(letter) + 1;
    }
    return letter as u32 - ('A' as u32) + ALPHABET_LENGTH + 1
}

/// Rucksack Reorganization
/// https://adventofcode.com/2022/day/3
pub fn solve_1(filename: &str) -> i32 {
    let reader = get_input_reader(filename, YEAR, PROBLEM);
    let mut result = 0;

    for raw_line in reader.lines() {
        match raw_line {
            Ok(line) => {
                if line == "" { continue }
                let items: Vec<char> = line.chars().collect();

                let mut rucksack_a = HashSet::new();
                let rucksack_size = items.len() / 2;

                for i in 0..rucksack_size {
                    rucksack_a.insert(items[i]);
                }

                let mut prioritised_items = HashSet::new();
                for i in 0..rucksack_size {
                    let item = items[rucksack_size+i];
                    if rucksack_a.contains(&item) && !prioritised_items.contains(&item) {
                        result += get_priority_value(item);
                        prioritised_items.insert(item);
                    }
                }
            },
            Err(error) => panic!("{}", error)
        };
    }
    return result as i32;
}

pub fn solve_2(filename: &str) -> i32 {
    let reader = get_input_reader(filename, YEAR, PROBLEM);
    let mut result = 0;

    let mut iter = reader.lines();

    while let Some(raw_line) = iter.next() {
        match raw_line {
            Ok(line) => {
                if line == "" { continue; }
                let line_b = iter.next().unwrap().unwrap();
                let rucksack_b: HashSet<char> = line_b.chars().collect();

                let line_c = iter.next().unwrap().unwrap();
                let rucksack_c: HashSet<char> = line_c.chars().collect();

                for elem in line.chars() {
                    if rucksack_b.contains(&elem) && rucksack_c.contains(&elem) {
                        result += get_priority_value(elem) as i32;
                        break;
                    }
                }
            },
            Err(error) => panic!("{}", error)
        };
    }
    return result;
}


#[cfg(test)]
mod tests {
    use super::solve_1;
    use super::solve_2;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve_1(input_file);
        assert_eq!(result, 7428);
    }

    #[test]
    fn part_2() {
        let input_file = "sample.txt";
        let result = solve_2(input_file);
        assert_eq!(result, 2650);
    }
}
