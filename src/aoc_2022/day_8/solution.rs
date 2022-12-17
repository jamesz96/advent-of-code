use std::io::BufRead;
use crate::{util::file::get_input_reader, aoc_2022::constants::YEAR};

const PROBLEM: &str = "day_8";

/// Treetop Tree House
/// https://adventofcode.com/2022/day/8
pub fn solve_part_1(filename: &str) -> i32 {
    let reader = get_input_reader(filename, YEAR, PROBLEM);
    let mut tree_table: Vec<Vec<i32>> = vec![];

    for raw_line in reader.lines() {
        match raw_line {
            Ok(line) => {
                if line == "" { continue; }
                let col_elements: Vec<&str> = line.split("").collect();
                let iter = col_elements[1..col_elements.len()-1].iter();
                tree_table.push(iter.map(|letter| letter.parse::<i32>().unwrap()).collect())
            },
            Err(_error) => continue,
        }
    }

    let length = tree_table[0].len();

    let mut spotted_table: Vec<Vec<bool>> = vec![vec![false; tree_table.len()]; tree_table.len()];

    // Left to right
    for i in 0..length {
        let mut max_height = -1;
        for j in 0..length {
            if tree_table[i][j] > max_height {
                spotted_table[i][j] = true;
                max_height = tree_table[i][j]
            }
        }
    }

    // right to left
    for i in 0..length {
        let mut max_height = -1;
        for j in (0..length).rev() {
            if tree_table[i][j] > max_height {
                spotted_table[i][j] = true;
                max_height = tree_table[i][j]
            }
        }
    }

    // top to bottom
    for i in 0..length {
        let mut max_height = -1;
        for j in 0..length {
            if tree_table[j][i] > max_height {
                spotted_table[j][i] = true;
                max_height = tree_table[j][i]
            }
        }
    }

    // bottom to top
    for i in 0..length {
        let mut max_height = -1;
        for j in (0..length).rev() {
            if tree_table[j][i] > max_height {
                spotted_table[j][i] = true;
                max_height = tree_table[j][i]
            }
        }
    }

    let mut result = 0;
    for i in 0..length {
        for j in 0..length {
            if spotted_table[i][j] {
                result += 1
            }
        }
    }
    return result;
}

fn scenic_score(table: &Vec<Vec<i32>>, pos: (usize, usize)) -> i32 {
    let num_rows = table.len();
    let num_cols = table[0].len();

    let (mut up, mut down, mut left, mut right) = (0, 0, 0, 0);
    let (x, y) = pos;
    let height = table[x][y];

    for i in x+1..num_rows {
        down += 1;
        if height <= table[i][y] { break; }
    }

    for i in (0..x).rev() {
        up += 1;
        if height <= table[i][y] { break; }
    }

    for i in y+1..num_cols {
        right += 1;
        if height <= table[x][i] { break; }
    }

    for i in (0..y).rev() {
        left += 1;
        if height <= table[x][i] { break; }
    }

    return up * down * left * right;
}

pub fn solve_part_2(filename: &str) -> i32 {
    let reader = get_input_reader(filename, YEAR, PROBLEM);
    let mut tree_table: Vec<Vec<i32>> = vec![];

    for raw_line in reader.lines() {
        match raw_line {
            Ok(line) => {
                if line == "" { continue; }
                let col_elements: Vec<&str> = line.split("").collect();
                let iter = col_elements[1..col_elements.len()-1].iter();
                tree_table.push(iter.map(|letter| letter.parse::<i32>().unwrap()).collect())
            },
            Err(_error) => continue,
        }
    }

    let length = tree_table[0].len();
    let mut result = 0;

    for i in 0..length {
        let width = tree_table[i].len();
        for j in 0..width {
            let pos = (i, j);
            let score = scenic_score(&tree_table, pos);
            result = std::cmp::max(result, score);
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::solve_part_1;
    use super::solve_part_2;

    #[test]
    fn test_1() {
        let input_file = "sample.txt";
        let result = solve_part_1(input_file);
        assert_eq!(result, 1733);
    }

    #[test]
    fn test_2() {
        let input_file = "sample.txt";
        let result = solve_part_2(input_file);
        assert_eq!(result, 284648);
    }
}
