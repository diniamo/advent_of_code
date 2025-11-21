#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|pair| {
                    (
                        pair.0.parse::<u32>().unwrap(),
                        pair.1.parse::<u32>().unwrap(),
                    )
                })
                .unwrap()
        })
        .unzip()
}

#[aoc(day1, part1)]
fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut left = input.0.clone();
    let mut right = input.1.clone();

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    input
        .0
        .iter()
        .map(|&left| left * (input.1.iter().filter(|&&right| right == left).count() as u32))
        .sum()
}
