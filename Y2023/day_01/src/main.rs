use common::file_access;

fn part_1(input: Vec<String>) {
    let sum = input.iter().fold(0, |sum, line| {
        let mut digits = String::from(line);
        digits.retain(|c| c.is_numeric());

        let left_int = digits.chars().next().unwrap();
        let right_int = digits.chars().rev().next().unwrap();

        let line_value = format!("{}{}", left_int, right_int);

        return sum + line_value.parse::<i32>().unwrap();
    });

    println!("Solution to part 1: {}", sum);
}

fn main() {
    let input = file_access::read_lines("../inputs/day_01.txt");
    part_1(input);
}
