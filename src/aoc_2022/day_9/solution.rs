use std::{fs::File, io::{Error, BufReader, BufRead}, collections::HashSet};
use crate::{util::file::get_current_dir, aoc_2022::constants};

const PROBLEM: &str = "day_9";

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Pos { x: i32, y: i32 }
struct Update { head: Pos, tail: Pos }
enum Direction { UP, DOWN, RIGHT, LEFT }

fn is_linked(a: &Pos, b: &Pos) -> bool {
    let x_diff = i32::abs(a.x - b.x);
    let y_diff = i32::abs(a.y - b.y);

    return x_diff <= 1 && y_diff <= 1
}

fn update(head: &Pos, tail: &Pos, direction: Direction, magnitude: i32, visited: &mut HashSet<Pos>) -> Update {
    let mut head_result_x = head.x;
    let mut head_result_y = head.y;
    let mut tail_result_x = tail.x;
    let mut tail_result_y = tail.y;

    let mut prev_head_result_x = head.x;
    let mut prev_head_result_y = head.y;

    for _ in 0..magnitude {
        match direction {
            Direction::UP => head_result_y += 1,
            Direction::DOWN => head_result_y -= 1,
            Direction::RIGHT => head_result_x += 1,
            Direction::LEFT => head_result_x -= 1,
        }

        if !is_linked(
            &Pos{ x: head_result_x, y: head_result_y },
            &Pos{ x: tail_result_x, y: tail_result_y }
        ) {
            tail_result_x = prev_head_result_x;
            tail_result_y = prev_head_result_y;
        }

        visited.insert(Pos { x: tail_result_x, y: tail_result_y });

        prev_head_result_x = head_result_x;
        prev_head_result_y = head_result_y;
    }

    Update {
        head: Pos { x: head_result_x, y: head_result_y },
        tail: Pos { x: tail_result_x, y: tail_result_y },
    }
}

fn parse_direction(input: &str) -> Direction {
    let result = match input {
        "L" => Direction::LEFT,
        "R" => Direction::RIGHT,
        "U" => Direction::UP,
        "D" => Direction::DOWN,
        _ => panic!("Cannot parse direction")
    };
    return result;
}

/// Rope Bridge
/// https://adventofcode.com/2022/day/9
pub fn solve(filename: String) -> Result<i32, Error> {
    let current_dir = get_current_dir();
    let path = format!("{}/src/{}/{}/{}", current_dir, constants::YEAR.to_string(), PROBLEM.to_string(), filename);
    let file = File::open(path)?;

    let mut visited: HashSet<Pos> = HashSet::new();

    let mut head = Pos{ x: 0, y: 0 };
    let mut tail = Pos{ x: 0, y: 0 };

    visited.insert(tail);
    let reader = BufReader::new(file);

    for raw_line in reader.lines() {
        match raw_line {
            Ok(raw_line) => {
                if raw_line == "" { continue; }
                let col_elements: Vec<&str> = raw_line.split(" ").collect();

                let direction = parse_direction(col_elements[0]);
                let magnitude = col_elements[1].parse::<i32>().unwrap();

                let updated = update(&head, &tail, direction, magnitude, &mut visited);
                head = updated.head;
                tail = updated.tail;
            },
            Err(_error) => continue,
        }
    }
    return Ok(visited.len() as i32);
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_9::solution::solve;

    #[test]
    fn test() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        match result {
            Ok(x) => assert_eq!(x, 6269),
            Err(_error) => return,
        }
    }
}
