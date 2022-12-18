use std::{io::{BufRead, BufReader, Lines}, collections::{VecDeque, HashSet}, fs::File};
use crate::{aoc_2022::constants::YEAR, util::{file::get_input_reader, char::get_letter_idx}};

const PROBLEM: &str = "day_12";
const ALPHABET_LENGTH: u32 = 26;

fn shortest_length_path(
    map: &Vec<Vec<u32>>,
    start: (usize, usize),
    end_fn: Box<dyn Fn(u32, (usize, usize)) -> bool>,
    is_reverse: bool,
) -> i32 {
    let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
    queue.push_back((start.0, start.1, 0));

    let num_rows = map.len();
    let num_cols = map[0].len();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while queue.len() > 0 {
        let curr = queue.pop_front().unwrap();
        let (x, y, l) = curr;

        let pos = (x, y);
        let curr_height = map[x][y];

        if end_fn(curr_height, pos) { return l; }

        if visited.contains(&pos) { continue; }
        let mut nachbarn: Vec<(usize, usize)> = vec![];

        if x >= 1 && (
        (!is_reverse && curr_height + 1 >= map[x-1][y]) || (is_reverse && curr_height - 1 <= map[x-1][y])) {
            nachbarn.push((x-1, y));
        }

        if y >= 1 && (
        (!is_reverse && curr_height + 1 >= map[x][y-1]) || (is_reverse && curr_height - 1 <= map[x][y-1])) {
            nachbarn.push((x, y-1));
        }

        if x + 1 < num_rows && (
        (!is_reverse && curr_height + 1 >= map[x+1][y]) || (is_reverse && curr_height - 1 <= map[x+1][y])) {
            nachbarn.push((x+1, y));
        }

        if y + 1 < num_cols && (
        (!is_reverse && curr_height + 1 >= map[x][y+1]) || (is_reverse && curr_height - 1 <= map[x][y+1])) {
            nachbarn.push((x, y+1));
        }

        for nachbar in nachbarn.iter() {
            let next = (nachbar.0, nachbar.1, l+1);
            queue.push_back(next);
        }

        visited.insert(pos);
    }

    return -1;
}

struct MapData {
    map: Vec<Vec<u32>>,
    end: (usize, usize),
    start: (usize, usize),
}

fn build_map(iter: &mut Lines<BufReader<File>>) -> MapData {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut map: Vec<Vec<u32>> = vec![];

    for (row, raw_line) in iter.enumerate() {
        match raw_line {
            Ok(line) => {
                if line == "" { continue; }
                let mut row_elements: Vec<u32> = vec![];
                for (col, c) in line.chars().enumerate() {
                    match c {
                        'S' => {
                            start = (row, col);
                            row_elements.push(0);
                        },
                        'E' => {
                            end = (row, col);
                            row_elements.push(ALPHABET_LENGTH - 1);
                        },
                        _ => {
                            row_elements.push(get_letter_idx(c));
                        }
                    }
                }
                map.push(row_elements);
            },
            Err(error) => panic!("{}", error),
        };
    }

    MapData { map, end, start }
}

/// Hill Climbing Algorithm
/// https://adventofcode.com/2022/day/12
pub fn solve_part_1(filename: &str) -> i32 {
    let reader = get_input_reader(filename, YEAR, PROBLEM);
    let mut iter = reader.lines();

    let MapData { map, end, start } = build_map(&mut iter);
    let end_case = move |_height: u32, pos: (usize, usize)| pos == end;
    return shortest_length_path(&map, start, Box::new(end_case), false);
}

pub fn solve_part_2(filename: &str, input_char: char) -> i32 {
    let reader = get_input_reader(filename, YEAR, PROBLEM);
    let mut iter = reader.lines();

    let MapData { map, end, start: _ } = build_map(&mut iter);
    let end_case = move |height: u32, _pos: (usize, usize)| height == get_letter_idx(input_char);

    return shortest_length_path(&map, end, Box::new(end_case), true);
}


#[cfg(test)]
mod tests {
    use super::solve_part_1;
    use super::solve_part_2;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve_part_1(input_file);
        assert_eq!(result, 383);
    }

    #[test]
    fn part_2_test() {
        let input_file = "test.txt";
        let result = solve_part_2(input_file, 'a');
        assert_eq!(result, 29);
    }

    #[test]
    fn part_2() {
        let input_file = "sample.txt";
        let result = solve_part_2(input_file, 'a');
        assert_eq!(result, 377);
    }
}
