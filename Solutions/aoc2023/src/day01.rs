use hashbrown::HashMap;

type Matchers = HashMap<String, u32>;

fn sum_first_last_digits(input: &str, matchers: &Matchers) -> String {
    let sum = input.lines().fold(0, |sum, line| {
        let mut left: Option<u32> = None;
        let mut right: u32 = 0;

        for (i, _) in line.chars().enumerate() {
            for (num, &val) in matchers.iter() {
                if line[i..].starts_with(num) {
                    left = left.or(Some(val));
                    right = val.to_owned();
                }
            }
        }

        return sum + left.unwrap() * 10 + right;
    });

    return sum.to_string();
}

pub fn run(input: &str) {
    let digit_matchers: Matchers = (0..=9).map(|i| (i.to_string(), i)).collect();
    let part_1 = sum_first_last_digits(input, &digit_matchers);
    println!("Part 1 solution: {}", part_1);

    let part2_matchers: Matchers = "one,two,three,four,five,six,seven,eight,nine"
        .split(',')
        .map(String::from)
        .zip(1..=9)
        .chain(digit_matchers)
        .collect();
    let part_2 = sum_first_last_digits(input, &part2_matchers);
    println!("Part 2 solution: {}", part_2);
}
