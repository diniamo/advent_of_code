enum Spring {
    Operational,
    Damaged,
    Unknown
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!()
        }
    }
}

type Input = Vec<(Vec<Spring>, Vec<usize>)>;

#[aoc_generator(day12)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|s| (
            s.0.chars().map(|c| c.into()).collect(),
            s.1.split(',').map(|n| n.parse::<usize>().unwrap()).collect()
        ))
        .collect()
}

fn get_suitable_clusters(springs: &Vec<Spring>, start: usize, size: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = Vec::new();

    for i in start..springs.len() {
        if i + size >= springs.len() { break; }
        // if matches!(springs[i], Spring::Unknown) &&
        //     springs[(i + 1)..(i + size + 1)].iter().take_while(|s| matches!(s, Spring::Damaged)).count() + 1 == size {
        // if springs[i..=(i + size)]
        //     vec.push(i);
        // }
    }

    vec
}

fn recurse_springs(springs: &Vec<Spring>, damaged_remaining: &[usize], start: usize, sum: u32) -> u32 {
    if damaged_remaining.is_empty() {
        return sum + 1;
    }

    let suitable = get_suitable_clusters(springs, start, damaged_remaining[0]);
    sum + suitable
        .iter()
        .map(|s| {
            recurse_springs(springs, &damaged_remaining[1..], *s + damaged_remaining[0], sum)
        })
        .sum::<u32>()
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|(springs, damaged)| recurse_springs(springs, damaged, 0, 0))
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &Input) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }

    const EXAMPLE: &str = "???.### 1,1,3";
}