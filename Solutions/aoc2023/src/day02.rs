fn find_rgb_max(line: &str) -> (i32, i32, i32) {
    let max_red =
        line.rsplit(" red").skip(1)
        .map(|e| {
            e.split(" ").last().unwrap().parse::<i32>().unwrap()
        }).max().unwrap_or(0);

    let max_blue =
        line.rsplit(" blue").skip(1)
            .map(|e| {
                e.split(" ").last().unwrap().parse::<i32>().unwrap()
            }).max().unwrap_or(0);

    let max_green =
        line.rsplit(" green").skip(1)
            .map(|e| {
                e.split(" ").last().unwrap().parse::<i32>().unwrap()
            }).max().unwrap_or(0);

    (max_red, max_green, max_blue)
}

fn part_1(input: &str) -> i32 {
    let poss_games = input.lines().filter(|line| {
        let rgb_max = find_rgb_max(line);
        rgb_max.0 <= 12 && rgb_max.1 <= 13 && rgb_max.2 <= 14
    });

    poss_games.map(|line| {
        line.split(' ')
            .collect::<Vec<&str>>()[1]
            .replace(':', "")
            .parse::<i32>()
            .unwrap()
    }).sum()
}

fn part_2(input: &str) -> i32 {
    input.lines().map(|line| {
        let rgb_max = find_rgb_max(line);
        rgb_max.0 * rgb_max.1 * rgb_max.2
    }).sum()
}

pub fn run(input: &str) {
    println!("Part 1 solution: {}", part_1(input));
    println!("Part 2 solution: {}", part_2(input));
}
