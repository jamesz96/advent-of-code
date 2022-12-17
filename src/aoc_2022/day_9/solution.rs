use std::{io::BufRead, collections::HashSet};
use crate::{util::file::get_input_reader, aoc_2022::constants::YEAR};

const PROBLEM: &str = "day_9";
const DEBUG: bool = true;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Pos { x: usize, y: usize}
#[derive(Debug)]
enum Direction { UP, DOWN, RIGHT, LEFT }

fn is_linked(a: &Pos, b: &Pos) -> bool {
    let x_diff = if a.x > b.x { a.x - b.x } else { b.x - a.x };
    let y_diff = if a.y > b.y { a.y - b.y } else { b.y - a.y };

    return x_diff <= 1 && y_diff <= 1
}

fn cascade(rope: &mut Vec<Pos>, pre_update_next: Pos) {
    let mut next = pre_update_next;

    for i in (0..rope.len()-1).rev() {
        let new_next = rope[i+1];
        let curr = rope[i];

        if !is_linked(&curr, &new_next) {
            rope[i] = next;
            next = curr;
        }
    }
}

fn update(
    rope: &mut Vec<Pos>,
    direction: Direction,
    magnitude: i32,
    visited: &mut HashSet<Pos>
) {
    let head_idx = rope.len() - 1;
    for _ in 0..magnitude {
        print_visited(&visited, &rope);
        let head = rope[head_idx];
        let mut new_head = head;

        match direction {
            Direction::UP => new_head.x += 1,
            Direction::DOWN => new_head.x -= 1,
            Direction::RIGHT => new_head.y += 1,
            Direction::LEFT => new_head.y -= 1,
        }

        rope[head_idx] = new_head;

        cascade(rope, head);
        visited.insert(rope[0]);
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

fn print_visited(visited: &HashSet<Pos>, rope: &Vec<Pos>) {
    let canvas_size = 26;
    let mut table = vec![vec!['.'; 50]; 50];
    let x_offset: usize = 13;
    let y_offset: usize = 13;
    for i in (x_offset..canvas_size+x_offset).rev() {
        for j in y_offset..canvas_size+y_offset {
            let p = Pos{ x: i - x_offset, y: j - y_offset };
            if visited.contains(&p) { table[i][j] = '#'; }
        }
    }

    for (idx, p) in rope.iter().rev().enumerate() {
        let x = (p.x + x_offset) as usize;
        let y = (p.y + y_offset) as usize;
        if p.x == 0 && p.y == 0 { table[x][y] = 's'; continue; }
        table[x][y] = std::char::from_digit(idx as u32, 10).unwrap();
    }

    for row in table.iter().rev() {
        let out: String = row.iter().collect();
        println!("{}", out);
    }

    println!("===========\n\n")
}

/// Rope Bridge
/// https://adventofcode.com/2022/day/9
pub fn solve(filename: &str, rope_len: i32) -> usize {
    let reader = get_input_reader(filename, YEAR, PROBLEM);

    let mut visited: HashSet<Pos> = HashSet::new();
    let start = Pos{x: 0, y: 0};
    visited.insert(start);
    let mut rope = vec![];

    for _ in 0..rope_len {
        rope.push(start);
    }

    for raw_line in reader.lines() {
        match raw_line {
            Ok(raw_line) => {
                if raw_line == "" { continue; }
                let col_elements: Vec<&str> = raw_line.split(" ").collect();

                let direction = parse_direction(col_elements[0]);
                let magnitude = col_elements[1].parse::<i32>().unwrap();

                update(&mut rope, direction, magnitude, &mut visited);
            },
            Err(_error) => continue,
        }
    }
    return visited.len()
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_9::solution::solve;

    #[ignore]
    #[test]
    fn test() {
        let input_file = "test.txt";
        let result = solve(input_file, 2);
        assert_eq!(result, 13);
    }

    #[ignore]
    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file, 2);
        assert_eq!(result, 6269);
    }

    #[ignore]
    #[test]
    fn part_2_a() {
        let input_file = "test.txt";
        let result = solve(input_file, 10);
        assert_eq!(result, 1);
    }

    #[test]
    fn part_2_b() {
        let input_file = "test_2.txt";
        let result = solve(input_file, 10);
        assert_eq!(result, 36);
    }
}
