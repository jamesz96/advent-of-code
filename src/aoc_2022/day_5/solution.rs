use crate::util::file::read_input;
use crate::aoc_2022::constants;

const PROBLEM: &str = "day_5";

const TOKEN_SIZE: usize = 3;

fn parse_configuration(raw_config: &[&str], config: &mut Vec<Vec<char>>) {
    for line in raw_config {
        let mut chars = line.chars();
        let mut col_idx = 0;
        while let Some(c) = chars.next() {
            let mut letter = c;
            for i in 0..TOKEN_SIZE+1 {
                if i == 1 {
                    if letter != ' ' {
                        config[col_idx].push(letter);
                    }
                } else {
                    let result = chars.next();
                    match result {
                        Some(x) => letter = x,
                        None => break,
                    }
                }
            }
            col_idx += 1;
        }
    }
}

/// Camp Cleanup
/// https://adventofcode.com/2022/day/4
pub fn solve(filename: String) -> i32 {
    let contents = read_input(filename, constants::YEAR.to_string(), PROBLEM.to_string());
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut idx = 0;
    let mut num_columns = 0;

    while !lines[idx].starts_with(" move") {
        if lines[idx].starts_with(" 1") {
            let columns: Vec<&str> = lines[idx].split(" ").collect();
            num_columns = columns[columns.len()-1].parse::<usize>().unwrap();
        }
        idx += 1;
    }

    let mut configuration: Vec<Vec<char>> = vec![vec![]; num_columns];

    parse_configuration(&lines[..idx], &mut configuration);
    println!("{:?}", configuration);


    return 0;
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_5::solution::solve;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        assert_eq!(result, 538);
    }
}
