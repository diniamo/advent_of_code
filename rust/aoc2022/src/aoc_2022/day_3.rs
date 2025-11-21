use aoc2022::Day;


pub struct Day3 {}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
// s1 and s2 are always the same length
fn intersection(strs: Vec<&str>) -> char {
    for c in strs.first().unwrap().chars() {
        if strs[1..].iter().all(|line| line.contains(c)) {
            return c
        }
    }

    unreachable!()
}

impl Day for Day3 {
    fn day_number(&self) -> u8 {
        3
    }

    fn part1(&self, input: &String) -> String {
        let bags = input.trim().split('\n').map(|line| (&line[..(line.len()/2)], &line[(line.len()/2)..])).collect::<Vec<(&str, &str)>>();

        let mut prio_sum = 0;
        for bag in bags {
            prio_sum += ALPHABET.find(intersection(vec![bag.0, bag.1])).unwrap() + 1;
        }

        prio_sum.to_string()
    }

    fn part2(&self, input: &String) -> String {
        let lines = input.trim().split('\n').collect::<Vec<&str>>();

        let mut prio_sum = 0;
        for i in 0..lines.len() {
            if (i + 1) % 3 == 0 {
                prio_sum += ALPHABET.find(intersection(vec![lines[i - 2], lines[i - 1], lines[i]])).unwrap() + 1;
            }
        }

        prio_sum.to_string()
    }
}

fn intersection_bitwise(compartments: &[&str]) -> usize {
    compartments.iter().map(|compartment| {
        let mut mask = 0usize;

        for item in compartment.chars() {
            // Add 1 because the priorities are indexed from 1
            let priority = ALPHABET.find(|c| c == item).unwrap() + 1;
            mask |= 1 << priority;
        }

        mask
    }).reduce(|accumulator, element| accumulator & element).unwrap()
}

impl Day3 {
    fn part1_bitwise(&self, input: &String) -> String {
        input.lines().map(|line| (&line[..(line.len()/2)], &line[(line.len()/2)..])).map(|bag| intersection_bitwise(&[bag.0, bag.1]).ilog2()).sum::<u32>().to_string()
    }

    fn part2_bitwise(&self, input: &String) -> String {
        input.lines().collect::<Vec<&str>>().chunks(3).map(|triplet| intersection_bitwise(triplet).ilog2()).sum::<u32>().to_string()
    }
}
