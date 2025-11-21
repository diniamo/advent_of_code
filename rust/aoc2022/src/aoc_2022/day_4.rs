use aoc2022::Day;


pub struct Day4 {}

impl Day for Day4 {
    fn day_number(&self) -> u8 {
        4
    }

    fn part1(&self, input: &String) -> String {
        // let pairs: Vec<((u8, u8), (u8, u8))> = input.trim().split('\n').map(|line| {
        //     let split = line.split_once(',').unwrap();
        //
        //     let e1_split = split.0.split_once('-').unwrap();
        //     let e1 = (e1_split.0.parse::<u8>().unwrap(), e1_split.1.parse::<u8>().unwrap());
        //
        //     let e2_split = split.1.split_once('-').unwrap();
        //     let e2 = (e2_split.0.parse::<u8>().unwrap(), e2_split.1.parse::<u8>().unwrap());
        //
        //     ((e1.0, e1.1), (e2.0, e2.1))
        // }).collect();
        let pairs: Vec<((u8, u8), (u8, u8))> = input.trim().split('\n')
            .map(|line| line.split_once(',').unwrap())
            .map(|(a, b)| (a.split_once('-').unwrap(), b.split_once('-').unwrap()))
            .map(|((a, b), (c, d))| (
                (a.parse().unwrap(), b.parse().unwrap()),
                (c.parse().unwrap(), d.parse().unwrap())
            ))
            .collect();

        let mut count = 0;
        for ((x1, x2), (y1, y2)) in pairs {
            // Check if either contains the other
            if (x1 <= y1 && x2 >= y2) || (y1 <= x1 && y2 >= x2) {
                count += 1;
            }
        }

        count.to_string()
    }

    fn part2(&self, input: &String) -> String {
        let pairs: Vec<((u8, u8), (u8, u8))> = input.trim().split('\n')
            .map(|line| line.split_once(',').unwrap())
            .map(|(a, b)| (a.split_once('-').unwrap(), b.split_once('-').unwrap()))
            .map(|((a, b), (c, d))| (
                (a.parse().unwrap(), b.parse().unwrap()),
                (c.parse().unwrap(), d.parse().unwrap())
            ))
            .collect();

        let mut count = 0;
        for ((x1, x2), (y1, y2)) in pairs {
            // (StartA <= EndB) and (EndA >= StartB)
            if x1 <= y2 && x2 >= y1 {
                count += 1;
            }
        }

        count.to_string()
    }
}
