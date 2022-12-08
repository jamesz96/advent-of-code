use std::{io::{BufReader, Error, BufRead, Lines}, fs::File, collections::HashMap};
use crate::{aoc_2022::constants, util::file::get_current_dir};

const PROBLEM: &str = "day_7";

struct Directory<'a> {
    name: Box<&'a str>,
    subdirs: HashMap<&'a str, Box<Directory<'a>>>,
    parent: Option<Box<Directory<'a>>>,
    size: i32,
}

fn change_dir<'a>(dir: &'a Directory, destination: &str) -> &'a Directory<'a> {
    match destination {
        ".." => {
            match &dir.parent {
                Some(x) => return &(*x),
                None => return dir
            }
        }
        _ => {
            match dir.subdirs.get(destination) {
                Some(x) => return x,
                None => return dir
            }
        }
    }
}

fn compute_properties(dir: &Directory, iter: &mut Lines<BufReader<File>>) {
    while let Some(line) = iter.next() {
        match line {
            Ok(line) => {
                let splits: Vec<&str> = line.split(" ").collect();
                let entity_name = splits[1];
                match splits[0] {
                    "dir" => {
                        let new_dir = Directory { name: Box::new(entity_name), subdirs: HashMap::new(), parent: Some(dir), size: 0 };
                    }
                }

            }
            Err(_error) => continue,
        }

    }

}

fn build_filesystem(file: File) -> Directory<'static> {
    let root = Directory {
        name: Box::new("/"),
        subdirs: HashMap::new(),
        parent: None,
        size: 0,
    };
    let reader = BufReader::new(file);

    let mut curr = &root;

    let mut lines_iter = reader.lines();
    while let Some(line) = lines_iter.next() {
        match line {
            Ok(line) => {
                let splits: Vec<&str> = line.split(" ").collect();

                if splits[0] == "$" {
                    match splits[1] {
                        "cd" => {
                            if splits[2] == "/" { curr = &root }
                        }
                        _ => curr = change_dir(curr, splits[2])
                    }
                } else if splits[0] == "ls" {


                }
            },
            Err(_error) => continue,
        }
    }
    return root;
}


/// No Space Left On Device
/// https://adventofcode.com/2022/day/7
pub fn solve(filename: String) -> Result<i32, Error> {
    let current_dir = get_current_dir();
    let path = format!("{}/src/{}/{}/{}", current_dir, constants::YEAR.to_string(), PROBLEM.to_string(), filename);

    let file = File::open(path)?;

    let fs = build_filesystem(file);
    return Ok(0);
}


#[cfg(test)]
mod tests {
    use crate::aoc_2022::day_7::solution::solve;

    #[test]
    fn part_1() {
        let input_file = "sample.txt";
        let result = solve(input_file.to_string());
        assert_eq!(result, 1896);
    }
}
