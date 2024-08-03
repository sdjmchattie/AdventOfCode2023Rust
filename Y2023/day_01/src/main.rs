use common::file_access;
use regex::Regex;

fn part_1(input: Vec<String>) {
    let left_int_regex = Regex::new(r"^\D*(\d)").unwrap();
    let right_int_regex = Regex::new(r"(\d)\D*$").unwrap();

    let sum = input.iter().fold(0, |sum, line| {
        let left_int_str = left_int_regex.captures(line)
                                     .unwrap()
                                     .get(1)
                                     .unwrap()
                                     .as_str();
        let right_int_str = right_int_regex.captures(line)
                                     .unwrap()
                                     .get(1)
                                     .unwrap()
                                     .as_str();
        let line_value = format!("{}{}", left_int_str, right_int_str);

        return sum + line_value.parse::<i32>().unwrap();
    });

    println!("Solution to part 1: {}", sum);
}

fn main() {
    let input = file_access::read_lines("../inputs/day_01.txt");
    part_1(input);
}
