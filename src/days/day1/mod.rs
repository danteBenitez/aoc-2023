use std::fs;

pub fn read_file_lines<'a>(file_name: &'a str) -> Vec<String> {
    let contents = fs::read_to_string(file_name)
        .expect("File not found.")
        .split("\n")
        .map(|str| String::from(str))
        .collect();
    return contents;
}