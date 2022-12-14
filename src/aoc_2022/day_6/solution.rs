use std::{collections::{VecDeque, HashMap}, io::Read};

use crate::{util::file::get_input_reader, aoc_2022::constants::YEAR};

const PROBLEM: &str = "day_6";

pub const BUFFER_SIZE_START_OF_PACKET: usize = 4;
pub const BUFFER_SIZE_START_OF_MESSAGE: usize = 14;

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
pub fn solve(filename: &str, buffer_size: usize) -> i32 {
    let mut reader = get_input_reader(filename, YEAR, PROBLEM);
    let mut contents: String = String::new();
    _ = reader.read_to_string(&mut contents);

    let mut map: HashMap<char, i32> = HashMap::new();
    let mut buffer: VecDeque<char> = VecDeque::new();

    let mut idx = 0;
    let mut chars = contents.chars();
    while let Some(letter) = chars.next() {
        if buffer.len() == buffer_size {
            if map.keys().len() == buffer_size { break; }
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
    use super::{solve, BUFFER_SIZE_START_OF_PACKET, BUFFER_SIZE_START_OF_MESSAGE};

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file, BUFFER_SIZE_START_OF_PACKET);
        assert_eq!(result, 1896);
    }

    #[test]
    fn part_2() {
        let input_file = "sample.txt";
        let result = solve(input_file, BUFFER_SIZE_START_OF_MESSAGE);
        assert_eq!(result, 3452);
    }
}
