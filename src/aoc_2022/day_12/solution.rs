use std::{io::{BufReader, Error, BufRead}, fs::File, collections::{VecDeque, HashSet}};
use crate::{aoc_2022::constants, util::{file::get_current_dir, char::get_letter_idx}};

const PROBLEM: &str = "day_12";
const ALPHABET_LENGTH: i32 = 26;

fn shortest_length_path(map: Vec<Vec<i32>>, start: (usize, usize), end: (usize, usize)) -> i32 {
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back((start.0, start.1, 0));

    let num_rows = map.len();
    let num_cols = map[0].len();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while queue.len() > 0 {
        let curr = queue.pop_front().unwrap();
        let (x, y, l) = curr;

        let pos = (x, y);
        if pos == end { return l.try_into().unwrap() }

        if visited.contains(&pos) { continue; }

        let curr_height = map[x][y];

        let mut nachbarn: Vec<(usize, usize)> = vec![];

        if x >= 1 && curr_height + 1 >= map[x-1][y] { nachbarn.push((x-1, y)); }
        if y >= 1 && curr_height + 1 >= map[x][y-1] { nachbarn.push((x, y-1)); }
        if x + 1 < num_rows && curr_height + 1 >= map[x+1][y] { nachbarn.push((x+1, y)); }
        if y + 1 < num_cols && curr_height + 1 >= map[x][y+1] { nachbarn.push((x, y+1)); }

        for nachbar in nachbarn.iter() {
            let next = (nachbar.0, nachbar.1, l+1);
            queue.push_back(next);
        }

        visited.insert(pos);
    }

    return -1;
}

/// Hill Climbing Algorithm
/// https://adventofcode.com/2022/day/12
pub fn solve(filename: String) -> Result<i32, Error> {
    let current_dir = get_current_dir();
    let path = format!("{}/src/{}/{}/{}", current_dir, constants::YEAR.to_string(), PROBLEM.to_string(), filename);

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<i32>> = vec![];

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (row, raw_line) in reader.lines().enumerate() {
        match raw_line {
            Ok(line) => {
                if line == "" { continue; }
                let mut row_elements = vec![];
                for (col, c) in line.chars().enumerate() {
                    match c {
                        'S' => {
                            start = (row, col);
                            row_elements.push(0);
                        },
                        'E' => {
                            end = (row, col);
                            row_elements.push(ALPHABET_LENGTH-1);
                        },
                        _ => {
                            row_elements.push(get_letter_idx(c) as i32);
                        }
                    }
                }
                map.push(row_elements);
            },
            Err(error) => panic!("{}", error),
        };
    }

    let result = shortest_length_path(map, start, end);
    return Ok(result);
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_12::solution::solve;

    #[test]
    fn test() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        match result {
            Ok(x) => assert_eq!(x, 383),
            Err(_error) => return,
        }
    }
}
