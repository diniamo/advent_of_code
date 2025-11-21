fn is_digit(c: &char) -> bool { c.is_ascii_digit() }

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut lr = l.chars().rev();

            format!("{}{}",
                    l.chars().find(is_digit).unwrap(),
                    lr.find(is_digit).unwrap()
            ).parse::<u32>().unwrap()
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let actual_input = input.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    part1(&actual_input)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(part1(&input), 142);
    }

    #[test]
    fn part2_example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

        assert_eq!(part2(&input), 281);
    }
}
