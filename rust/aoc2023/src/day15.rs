use regex::Regex;

#[aoc_generator(day15)]
fn parse(input: &str) -> Vec<String> {
    input
        .replace('\n', ",")
        .split(',')
        .map(String::from)
        .collect()
}

fn hash(string: &str) -> u8 {
    let mut current_value = 0u16;
    for c in string.chars() {
        current_value = (current_value + c as u16) * 17 % 256;
    }
    current_value as u8
}

#[aoc(day15, part1)]
fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|s| hash(s) as u32)
        .sum::<u32>()
}

#[aoc(day15, part2)]
fn part2(input: &[String]) -> u32 {
    let re = Regex::new(r"(a-z+)[=\-](\d)").unwrap();

    for step in input {
        // re.captures
    }

    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 1320);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 145);
    }

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
}