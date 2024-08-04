struct NumMatch<'a> {
    num: &'a str,
    val: u32,
}

fn sum_first_last_digits(input: &str, matchers: Vec<NumMatch>) -> String {
    let sum = input.lines().fold(0, |sum, line| {
        let mut left: Option<u32> = None;
        let mut right: Option<u32> = None;

        for (i, _) in line.chars().enumerate() {
            for m in matchers.iter() {
                if line[i..].starts_with(m.num) {
                    left = left.or(Some(m.val));
                    right = Some(m.val);
                }
            }
        }

        return sum + left.unwrap() * 10 + right.unwrap();
    });

    return sum.to_string();
}

fn part_1(input: &str) -> String {
    let matchers = vec![
        NumMatch { num: "1", val: 1 },
        NumMatch { num: "2", val: 2 },
        NumMatch { num: "3", val: 3 },
        NumMatch { num: "4", val: 4 },
        NumMatch { num: "5", val: 5 },
        NumMatch { num: "6", val: 6 },
        NumMatch { num: "7", val: 7 },
        NumMatch { num: "8", val: 8 },
        NumMatch { num: "9", val: 9 },
    ];

    return sum_first_last_digits(input, matchers);
}

fn part_2(input: &str) -> String {
    let matchers = vec![
        NumMatch { num: "1", val: 1 },
        NumMatch { num: "2", val: 2 },
        NumMatch { num: "3", val: 3 },
        NumMatch { num: "4", val: 4 },
        NumMatch { num: "5", val: 5 },
        NumMatch { num: "6", val: 6 },
        NumMatch { num: "7", val: 7 },
        NumMatch { num: "8", val: 8 },
        NumMatch { num: "9", val: 9 },
        NumMatch { num: "one", val: 1 },
        NumMatch { num: "two", val: 2 },
        NumMatch {
            num: "three",
            val: 3,
        },
        NumMatch {
            num: "four",
            val: 4,
        },
        NumMatch {
            num: "five",
            val: 5,
        },
        NumMatch { num: "six", val: 6 },
        NumMatch {
            num: "seven",
            val: 7,
        },
        NumMatch {
            num: "eight",
            val: 8,
        },
        NumMatch {
            num: "nine",
            val: 9,
        },
    ];

    return sum_first_last_digits(input, matchers);
}

pub fn run(input: &str) {
    println!("Part 1 solution: {}", part_1(input));
    println!("Part 2 solution: {}", part_2(input));
}
