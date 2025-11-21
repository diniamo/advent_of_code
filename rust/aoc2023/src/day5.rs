use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Copy, Clone)]
struct DestinationMap {
    source_start: u32,
    destination_start: u32,
    length: u32
}

type AlmanacMap = HashMap<String, Vec<DestinationMap>>;
#[derive(Clone)]
struct Almanac {
    seeds: Vec<u32>,
    map: AlmanacMap,
    states: HashMap<String, String>
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Almanac {
    let mut iter = input.lines();

    let seeds: Vec<u32> = iter
        .next().unwrap()
        .split_once(": ").unwrap().1
        .split(' ').map(|n| {
            n.parse::<u32>().unwrap()
        })
        .collect();
    iter.next();

    Almanac {
        seeds,
        map: iter
            .collect::<Vec<&str>>()
            .join("\n")
            .split("\n\n")
            .map(|s| {
                let mut lines = s.lines();
                let (from, _to) = lines.next().unwrap().split_once(' ').unwrap().0.split_once("-to-").unwrap();

                let mut ranges: Vec<DestinationMap> = Vec::new();
                for line in lines {
                    let numbers: Vec<u32> = line.split(' ').map(|n| n.parse::<u32>().unwrap()).collect();
                    let dest = numbers[0];
                    let src =  numbers[1];
                    let len =  numbers[2];

                    ranges.push(DestinationMap {
                        source_start: src,
                        destination_start: dest,
                        length: len
                    });
                }

                (String::from(from), ranges)
            })
            .collect(),
        states: HashMap::from([
            ("seed", "soil"),
            ("soil", "fertilizer"),
            ("fertilizer", "water"),
            ("water", "light"),
            ("light", "temperature"),
            ("temperature", "humidity"),
            ("humidity", "location"),
        ].map(|p| (String::from(p.0), String::from(p.1))))
    }
}


#[aoc(day5, part1)]
fn part1(input: &Almanac) -> u32 {
    input.seeds.par_iter()
        .map(|s| {
            let mut current_state = "seed";
            let mut current_number = *s;

            while current_state != "location" {
                let ranges = input.map.get(current_state).unwrap();

                for range in ranges {
                    if range.source_start <= current_number && current_number < range.source_start + range.length {
                        current_number = range.destination_start + (current_number - range.source_start);
                        break;
                    }
                }

                current_state = input.states.get(current_state).unwrap();
            }

            current_number
        })
        .min().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &Almanac) -> u32 {
    let actual_seeds = input.seeds
        .iter().step_by(2)
        .zip(input.seeds.iter().skip(1).step_by(2))
        .flat_map(|p| (*p.0)..(*p.0 + *p.1))
        .collect::<Vec<u32>>();

    part1(&Almanac {
        seeds: actual_seeds,
        ..(input.clone())
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 46);
    }

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
}