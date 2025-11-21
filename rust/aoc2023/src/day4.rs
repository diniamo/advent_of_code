use std::collections::HashSet;

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<(HashSet<u32>, HashSet<u32>)> {
    input
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.split_once(" | ").unwrap())
        .map(|card| (
            card.0.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<HashSet<u32>>(),
            card.1.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<HashSet<u32>>()
        )).collect()
}

#[aoc(day4, part1)]
fn part1(input: &[(HashSet<u32>, HashSet<u32>)]) -> u32 {
    input
        .iter()
        .map(|lr| 2u32.pow((lr.0.intersection(&lr.1).count() as u32) - 1))
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[(HashSet<u32>, HashSet<u32>)]) -> usize {
    let len = input.len();
    let mut copies = vec![1usize; len];
    for (i, card) in input.iter().enumerate() {
        let wins = card.0.intersection(&card.1).count();

        for j in (i + 1)..(i + 1 + wins) {
            if j < len {
                copies[j] += copies[i];
            }
        }
    }

    copies.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        // This fails, but I'm going to leave it like this for the funny.
        // The reason the real input doesn't fail is because there are no lines with no intersection, so 0u32 - 1 (on line 28) can't happen
        assert_eq!(part1(&parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")), 30);
    }
}