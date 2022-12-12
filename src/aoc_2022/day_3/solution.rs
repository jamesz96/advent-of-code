use std::collections::HashSet;

use crate::util::char::get_letter_idx;
use crate::util::file::read_input;
use crate::aoc_2022::constants;

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
pub fn solve(filename: String) -> i32 {
    let contents = read_input(filename, constants::YEAR.to_string(), PROBLEM.to_string());
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut result = 0;

    for line in lines {
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
    }
    return result as i32;
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_3::solution::solve;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        assert_eq!(result, 7428);
    }
}
