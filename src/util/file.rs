use std::fs;
use std::env;
use std::fs::File;
use std::io::BufReader;

pub fn get_current_dir() -> String {
    let result = env::current_dir();
    match result {
        Ok(result) => return result.into_os_string().into_string().unwrap(),
        Err(_) => panic!("Cannot find current directory in fs... where are you running this?")
    };
}

pub fn read_input(filename: String, year: String, problem: String) -> String {
    let current_dir = get_current_dir();
    let path = format!("{}/src/{}/{}/{}", current_dir, year, problem, filename);
    let contents = fs::read_to_string(path).expect("Cannot find file");
    return contents;
}

pub fn get_input_reader(filename: &str, year: &str, problem: &str) -> BufReader<File> {
    let current_dir = get_current_dir();
    let path = format!("{}/src/{}/{}/{}", current_dir, year, problem, filename);

    match File::open(path) {
        Ok(file) => return BufReader::new(file),
        Err(_error) => panic!("Cannot open file"),
    };
}
