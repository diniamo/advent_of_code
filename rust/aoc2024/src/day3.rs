use regex::Regex;

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    pattern.captures_iter(input).map(|captures| {
        let (_, [lhs, rhs]) = captures.extract();
        lhs.parse::<u32>().unwrap() * rhs.parse::<u32>().unwrap()
    }).sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let command_pattern = Regex::new(r"do(?:n't)?\(\)").unwrap();
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut dos = command_pattern.find_iter(input).map(|m| (m.end(), m.as_str().to_string())).collect::<Vec<_>>();
    dos.sort_by_key(|d| d.0);

    mul_pattern.captures_iter(input).filter_map(|captures| {
        let start = captures.get(0).unwrap().start();
        let command = dos.iter().filter(|i| i.0 <= start).last();

        if command.is_none() || command.unwrap().1 == "do()" {
            let (_, [lhs, rhs]) = captures.extract();
            Some(lhs.parse::<u32>().unwrap() * rhs.parse::<u32>().unwrap())
        } else {
            None
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), 161);
    }

    #[test]
    fn part2_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_do()mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), 48);
    }
}
