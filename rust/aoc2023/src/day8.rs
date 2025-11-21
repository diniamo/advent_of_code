use std::collections::HashMap;
use num::integer;
use regex::Regex;

type Input = (Vec<char>, HashMap<String, (String, String)>);

#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect();

    let re = Regex::new(r"[A-Z1-9]{3}").unwrap();
    (
        instructions,
        lines
            .skip(1)
            .map(|l| {
                let captures = re.captures_iter(l).map(|m| m.get(0).unwrap());
                (
                    captures.get(0).unwrap().as_str().to_string(),
                    (
                        captures.get(1).unwrap().as_str().to_string(),
                        captures.get(2).unwrap().as_str().to_string()
                    )
                )
            })
            .collect()
    )
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> u32 {
    find_z_node(input, "AAA", |n| n == "ZZZ")
}

fn find_z_node(input: &Input, start: &str, condition: fn(&str) -> bool) -> u32 {
    let instructions_len = input.0.len();

    let mut steps = 0u32;

    let mut current_node = start;
    let mut i = 0usize;
    while !condition(current_node) {
        if i > instructions_len - 1 {
            i = 0
        }

        current_node = if input.0[i] == 'L' {
            &input.1.get(current_node).unwrap().0
        } else {
            &input.1.get(current_node).unwrap().1
        };

        steps += 1;
        i += 1;
    }

    steps
}

#[aoc(day8, part2)]
fn part2_optimized(input: &Input) -> u64 {
    let z_nodes = input.1
        .keys()
        .filter_map(|n| {
            if n.ends_with('A') {
                Some(find_z_node(input, n, |n| n.ends_with('Z')) as u64)
            } else {
                None
            }
        })
        .collect::<Vec<u64>>();

    let mut lcm = integer::lcm(z_nodes[0], z_nodes[1]);
    let mut i = 2;
    while i < z_nodes.len() {
        lcm = integer::lcm(lcm, z_nodes[i]);
        i += 1;
    }

    lcm
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 6);
    }

    const EXAMPLE_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part2_example() {
        assert_eq!(part2_optimized(&parse(EXAMPLE_2)), 6);
    }
}