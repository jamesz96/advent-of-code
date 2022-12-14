use std::{io::{BufReader, BufRead, Lines}, fs::File, collections::HashMap, iter::Peekable};
use crate::{aoc_2022::constants::YEAR, util::file::get_input_reader};

const PROBLEM: &str = "day_7";
const SIZE_LIMIT: i32 = 100000;
const UNUSED_SPACE_REQUIRED: i32 = 30000000;
const CAPACITY: i32 = 70000000;

struct Node {
    children: Vec<String>,
    size: i32,
}

struct Arena {
    nodes: HashMap<String, Node>,
}

fn construct_node() -> Node {
    Node {
        children: vec![],
        size: 0,
    }
}

fn format_path(elements: &Vec<String>) -> String {
    if elements.len() == 1 && elements[0] == "/" { return String::from("/"); }
    return format!("/{}", elements[1..].join("/"));
}

fn change_dir(name: &String, stack: &mut Vec<String>) {
    match name.as_str() {
        ".." => _ = stack.pop(),
        _ => stack.push(name.to_string()),
    };
}

fn process_content(path: &String, iter: &mut Peekable<Lines<BufReader<File>>>, arena: &mut Arena) {
    while let Some(line) = iter.peek() {
        match line {
            Ok(content) => {
                let tokens: Vec<String> = content.split(" ").map(String::from).collect();
                match tokens[0].as_str() {
                    "$" => return,
                    "dir" => {
                        let key = match path.as_str() {
                            "/" => format!("/{}", tokens[1]),
                            _ => format!("{}/{}", path, tokens[1]),
                        };
                        let curr_node = arena.nodes.get_mut(path).unwrap();
                        curr_node.children.push(key.to_string());
                        arena.nodes.insert(key.to_string(), construct_node());
                    },
                    _ => {
                        let val = tokens[0].parse::<i32>().unwrap();
                        let mut curr_node = arena.nodes.get_mut(path).unwrap();
                        curr_node.size += val;
                    },
                };


            },
            Err(_error) => continue,
        };
        iter.next();
    }
}

fn dfs_update(item_key: &String, nodes: &HashMap<String, Node>, sizes: &mut HashMap<String, i32>) -> i32 {
    let ref_node = nodes.get(item_key).unwrap();
    let mut size = ref_node.size;

    for key in &ref_node.children {
        size += dfs_update(&key, nodes, sizes);
    }

    sizes.insert(item_key.to_string(), size);
    return size;
}


/// No Space Left On Device
/// https://adventofcode.com/2022/day/7
pub fn solve(filename: &str, is_update: bool) -> i32 {
    let reader = get_input_reader(filename, YEAR, PROBLEM);

    let mut stack: Vec<String> = vec![];
    let root = String::from("/");

    let mut arena = Arena { nodes: HashMap::new() };
    arena.nodes.insert(root.to_string(), construct_node());

    let mut iter = reader.lines().peekable();
    while let Some(line) = iter.next() {
        match line {
            Ok(command_str) => {
                if command_str == "" { continue; }
                let tokens: Vec<String> = command_str.split(" ").map(String::from).collect();
                if tokens[0] != "$" { continue; }

                match tokens[1].as_str() {
                    "cd" => change_dir(&tokens[2], &mut stack),
                    "ls" => process_content(&format_path(&stack), &mut iter, &mut arena),
                    _ => continue,
                }
            },
            Err(_) => continue,
        }
    };

    let mut size_map: HashMap<String, i32> = HashMap::new();
    dfs_update(&root, &arena.nodes, &mut size_map);


    match is_update {
        true => {
            let total_size_used = size_map.get(&root).unwrap();
            let unused_size = CAPACITY - total_size_used;
            let threshold = UNUSED_SPACE_REQUIRED - unused_size;

            let mut result = std::i32::MAX;
            for val in size_map.values() {
                if val >= &threshold {
                    result = std::cmp::min(*val, result);
                }
            }
            return result;
        }
        false => {
            let mut result = 0;
            for val in size_map.values() {
                if *val <= SIZE_LIMIT { result += val; }
            }
            return result;
        }
    };
}


#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file, false);
        assert_eq!(result, 1182909);
    }

    #[test]
    fn part_2() {
        let input_file = "sample.txt";
        let result = solve(input_file, true);
        assert_eq!(result, 2832508);
    }
}
