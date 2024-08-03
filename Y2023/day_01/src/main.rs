use common::file_access;

fn sum_first_last_digits(input: Vec<String>) -> String {
    let sum = input.iter().fold(0, |sum, line| {
        let mut digits = String::from(line);
        digits.retain(|c| c.is_numeric());

        let left_int = digits.chars().next().unwrap();
        let right_int = digits.chars().rev().next().unwrap();

        let line_value = format!("{}{}", left_int, right_int);

        return sum + line_value.parse::<i32>().unwrap();
    });

    return sum.to_string();
}

fn part_1(input: &Vec<String>) -> String {
    return sum_first_last_digits(input.to_vec());
}

static NUM_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn replace_written_digits(number: String, before: String) -> String {
    // Immediately return where number is fully consumed.
    if number.chars().count() == 0 {
        return before;
    }

    // Check for any NUM_WORDS at the start of the string and replace with
    // the equivalent digit (index + 1).
    let pos_num_word = NUM_WORDS
        .iter()
        .enumerate()
        .find(|&nw| number.starts_with(nw.1));
    if pos_num_word.is_some() {
        let num_word = pos_num_word.unwrap();
        let word_length = num_word.1.chars().count();
        return replace_written_digits(
            number[word_length..].to_string(),
            format!("{before}{}", num_word.0 + 1),
        );
    }

    let first_char = number.chars().next().unwrap();
    if first_char.is_numeric() {
        return replace_written_digits(number[1..].to_string(), format!("{before}{first_char}"));
    }

    // All other conditions unmet, so move on.
    return replace_written_digits(number[1..].to_string(), before);
}

fn part_2(input: &Vec<String>) -> String {
    let numeric_lines: Vec<String> = input
        .iter()
        .map(|line| {
            return replace_written_digits(line.to_string(), String::from(""));
        })
        .collect();

    return sum_first_last_digits(numeric_lines.to_vec());
}

fn main() {
    let input = file_access::read_lines("../inputs/day_01.txt");
    println!("Solution to part 1: {}", part_1(&input));
    println!("Solution to part 2: {}", part_2(&input));
}
