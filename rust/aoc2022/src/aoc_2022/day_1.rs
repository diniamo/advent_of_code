use aoc2022::Day;


pub struct Day1 {}

impl Day for Day1 {
    fn day_number(&self) -> u8 {
        1
    }

    fn part1(&self, input: &String) -> String {
        let mut values = Vec::new();

        let mut tmp = 0;
        for line in input.split('\n') {
            if line.is_empty() {
                values.push(tmp);
                tmp = 0;
            } else {
                tmp += line.parse::<u32>().unwrap();
            }
        }

        values.iter().max().unwrap().to_string()
    }

    fn part2(&self, input: &String) -> String {
        let mut values = Vec::new();

        let mut tmp = 0;
        for line in input.split('\n') {
            if line.is_empty() {
                values.push(tmp);
                tmp = 0;
            } else {
                tmp += line.parse::<u32>().unwrap();
            }
        }

        values.sort_unstable();
        values.reverse();
        values.iter().take(3).sum::<u32>().to_string()
    }
}
