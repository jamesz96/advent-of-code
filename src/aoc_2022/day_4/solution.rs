use crate::util::file::read_input;
use crate::aoc_2022::constants;

const PROBLEM: &str = "day_4";

struct Range {
    floor: i32,
    ceiling: i32,
}

fn build_range(data: &str) -> Range {
    let vals: Vec<&str> = data.split("-").collect();

    Range {
        floor: vals[0].parse::<i32>().unwrap(),
        ceiling: vals[1].parse::<i32>().unwrap(),
    }
}

/// is A (in its entirety) contained in B
fn contains(range_a: &Range, range_b: &Range) -> bool {
    return range_a.floor >= range_b.floor && range_a.ceiling <= range_b.ceiling;
}

/// Camp Cleanup
/// https://adventofcode.com/2022/day/4
pub fn solve(filename: String) -> i32 {
    let contents = read_input(filename, constants::YEAR.to_string(), PROBLEM.to_string());
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut result = 0;

    for line in lines {
        if line == "" { continue }
        let ranges: Vec<&str> = line.split(",").collect();

        let range_a = build_range(ranges[0]);
        let range_b = build_range(ranges[1]);

        if contains(&range_a, &range_b) || contains(&range_b, &range_a) {
            result += 1;
        }
    }
    return result as i32;
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_4::solution::solve;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        assert_eq!(result, 538);
    }
}
