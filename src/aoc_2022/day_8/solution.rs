use std::{fs::File, io::{Error, BufReader, BufRead}};
use crate::{util::file::get_current_dir, aoc_2022::constants};

const PROBLEM: &str = "day_8";

/// Treetop Tree House
/// https://adventofcode.com/2022/day/8
pub fn solve(filename: String) -> Result<i32, Error> {
    let current_dir = get_current_dir();
    let path = format!("{}/src/{}/{}/{}", current_dir, constants::YEAR.to_string(), PROBLEM.to_string(), filename);
    let file = File::open(path)?;

    let mut tree_table: Vec<Vec<i32>> = vec![];
    let reader = BufReader::new(file);

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
    return Ok(result);
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_8::solution::solve;

    #[test]
    fn test() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        match result {
            Ok(x) => assert_eq!(x, 1733),
            Err(_error) => return,
        }
    }
}
