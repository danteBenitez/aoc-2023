use std::fs;
use std::path::Path;

pub fn read_file_lines<'a>(file_name: &'a str) -> Vec<String> {
    let contents = fs::read_to_string(file_name)
        .expect("File not found.")
        .split("\n")
        .map(|str| String::from(str))
        .collect();
    return contents;
}

pub fn main() {
    let path = Path::new("src/inputs/day1.txt");
    let path = path.to_str().expect("Could not get path str.");
    let lines = read_file_lines(path);
    let mut total: usize = 0;
    // Extract two numbers for every line
    for line in lines.iter() {
        let number_str = line
            .as_bytes()
            .iter()
            .filter(|b| b.is_ascii_digit())
            .map(|b| Into::<char>::into(*b));

        let number_str = number_str.collect::<Vec<_>>();

        if number_str.len() == 0 {
            continue;
        }

        let first_n = number_str
            .get(0)
            .unwrap()
            .to_digit(10);
        let sec_n = number_str
            .get(number_str.len() - 1)
            .unwrap()
            .to_digit(10);

        let first_n = first_n.unwrap_or(0);
        let sec_n = sec_n.unwrap_or(0);

        let number = first_n * 10 + sec_n;
        total += number as usize;
    }

    println!("Total of calibration value: {}", total);
}