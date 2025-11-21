use aoc2022::Day;
use std::collections::VecDeque;


pub struct Day5 {}

impl Day for Day5 {
    fn day_number(&self) -> u8 {
        5
    }

    fn part1(&self, input: &String) -> String {
        let mut table: Vec<VecDeque<char>> = Vec::new();
        let mut moves: Vec<(u8, u8, u8)> = Vec::new();
        let mut is_past = false;
        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            if !is_past && !line.trim().starts_with('[') {
                is_past = true;
                continue;
            }

            if is_past {
                let mut split = line.split(' ');
                moves.push((
                    split.nth(1).unwrap().parse().unwrap(),
                    split.nth(1).unwrap().parse().unwrap(),
                    split.nth(1).unwrap().parse().unwrap()
                ));
            } else {
                for (i, c) in line.chars().enumerate() {
                    if i == 0 { continue; }

                    let index = (i - 1) / 4;
                    if table.len() == index {
                        table.push(VecDeque::new());
                    }

                    if (i - 1) % 4 == 0 && c != ' ' {
                        _ = table[index].push_front(c);
                    }
                }
            }
        }

        for m in moves {
            for _ in 0..m.0 {
                let to_add = (&mut table[usize::from(m.1 - 1)]).pop_back().unwrap();
                _ = (&mut table[usize::from(m.2 - 1)]).push_back(to_add);
            }
        }


        let mut top_chars = String::new();
        for column in &mut table {
            top_chars.push(column.pop_back().unwrap());
        }

        top_chars
    }

    fn part2(&self, input: &String) -> String {
        let mut table: Vec<VecDeque<char>> = Vec::new();
        let mut moves: Vec<(u8, u8, u8)> = Vec::new();
        let mut is_past = false;
        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            if !is_past && !line.trim().starts_with('[') {
                is_past = true;
                continue;
            }

            if is_past {
                let mut split = line.split(' ');
                moves.push((
                    split.nth(1).unwrap().parse().unwrap(),
                    split.nth(1).unwrap().parse().unwrap(),
                    split.nth(1).unwrap().parse().unwrap()
                ));
            } else {
                for (i, c) in line.chars().enumerate() {
                    if i == 0 { continue; }

                    let index = (i - 1) / 4;
                    if table.len() == index {
                        table.push(VecDeque::new());
                    }

                    if (i - 1) % 4 == 0 && c != ' ' {
                        _ = table[index].push_front(c);
                    }
                }
            }
        }

        for m in moves {
            let mut to_add: Vec<char> = Vec::new();
            for _ in 0..m.0 {
                to_add.push((&mut table[usize::from(m.1 - 1)]).pop_back().unwrap());
            }
            to_add.reverse();

            for c in to_add {
                _ = (&mut table[usize::from(m.2 - 1)]).push_back(c);
            }
        }


        let mut top_chars = String::new();
        for column in &mut table {
            top_chars.push(column.pop_back().unwrap());
        }

        top_chars
    }
}
