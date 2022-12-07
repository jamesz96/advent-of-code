use std::collections::{VecDeque, HashMap};

use crate::{util::file::read_input, aoc_2022::constants};

const PROBLEM: &str = "day_6";

const BUFFER_SIZE: usize = 4;

fn eject_item(map: &mut HashMap<char, i32>, key: char) {
    let mut quantity = 0;
    if let Some(x) = map.get(&key) {
        quantity = *x - 1;
    }

    if quantity == 0 {
        map.remove(&key);
    } else {
        map.insert(key, quantity);
    }
}

fn insert_item(map: &mut HashMap<char, i32>, key: char) {
    let mut quantity = 1;
    if let Some(x) = map.get(&key) {
        quantity = *x + 1;
    }
    map.insert(key, quantity);
}

/// Tuning Trouble
/// https://adventofcode.com/2022/day/6
pub fn solve(filename: String) -> i32 {
    let contents = read_input(filename, constants::YEAR.to_string(), PROBLEM.to_string());
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut buffer: VecDeque<char> = VecDeque::new();

    let mut idx = 0;
    let mut chars = contents.chars();
    while let Some(letter) = chars.next() {
        if buffer.len() == BUFFER_SIZE {
            if map.keys().len() == BUFFER_SIZE { break; }
            let element = buffer.pop_front();
            match element {
                Some(x) => eject_item(&mut map, x),
                None => continue
            };
        }

        insert_item(&mut map, letter);
        buffer.push_back(letter);
        idx += 1;
    }
    return idx;
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_6::solution::solve;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        assert_eq!(result, 1896);
    }
}
