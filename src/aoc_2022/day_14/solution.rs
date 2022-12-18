use std::io::BufRead;
use crate::{aoc_2022::constants::YEAR, util::file::get_input_reader};

const PROBLEM: &str = "day_14";
const RESERVOIR_WIDTH: usize = 650;
const RESERVOIR_HEIGHT: usize = 10;

struct Range {
    start: usize,
    end: usize,
}

fn build_range(a: usize, b: usize) -> Range {
    Range {
        start: if a < b { a } else { b },
        end: if a < b { b } else { a }
    }
}

fn construct(table: &mut [[char; RESERVOIR_WIDTH]; RESERVOIR_HEIGHT], geo: &Vec<(usize, usize)>) {
    for i in 0..geo.len()-1 {
        let (curr, next) = (geo[i], geo[i+1]);

        if curr.0 == next.0 {
            let Range { start, end } = build_range(curr.1, next.1);
            for col in start..end+1 {
                table[curr.0][col] = '#';
            }
        } else if curr.1 == next.1 {
            let Range { start, end } = build_range(curr.0, next.0);
            for row in start..end+1 {
                table[row][curr.1] = '#';
            }
        } else {
            panic!("Wir ein kleines Problem haben...");
        }

    }
}

fn print_reservoir(table: &[[char; RESERVOIR_WIDTH]; RESERVOIR_HEIGHT]) {
    for row in 0..table.len() {
        let s: String = table[row].iter().collect();
        println!("{}", s);
    }
}

fn string_to_point(raw_input: &str) -> (usize, usize) {
    println!("{}", raw_input);
    let splits: Vec<&str> = raw_input.split(",").collect();
    let x = splits[0].parse::<usize>().unwrap();
    let y = splits[1].parse::<usize>().unwrap();
    let (row, col) = (y, x);
    return (row, col);
}

/// Regolith Reservoir
/// https://adventofcode.com/2022/day/14
pub fn solve(filename: &str) -> i32 {
    let reader = get_input_reader(filename, YEAR, PROBLEM);

    let mut reservoir = [['.'; RESERVOIR_WIDTH]; RESERVOIR_HEIGHT];

    for raw_line in reader.lines() {
        match raw_line {
            Ok(line) => {
                let geo: Vec<(usize, usize)> = line
                    .split(" -> ")
                    .map(string_to_point)
                    .collect();

                construct(&mut reservoir, &geo);
            },
            Err(error) => panic!("{}", error),
        };
    }
    print_reservoir(&reservoir);
    return 0;
}


#[cfg(test)]
mod tests {
    use super::solve;

    #[ignore]
    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file);
        assert_eq!(result, 13);
    }

    #[test]
    fn part_1_test() {
        let input_file = "test.txt";
        let result = solve(input_file);
        assert_eq!(result, 13);
    }
}
