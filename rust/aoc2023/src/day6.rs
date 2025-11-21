use regex::Regex;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_whitespace().skip(1).map(|n| n.parse::<f32>().unwrap());
    let distances = lines.next().unwrap().split_whitespace().skip(1).map(|n| n.parse::<f32>().unwrap());
    let races = times.zip(distances).collect::<Vec<(f32, f32)>>();

    let mut product = 1usize;

    for race in races {
        let t = race.0;
        let d_max = race.1;

        let sqrt_term = f32::sqrt(t*t - 4f32*d_max);
        let x1 = (t + sqrt_term) / 2f32;
        let x2 = (t - sqrt_term) / 2f32;

        if x1 > x2 {
            product *= ((x2.floor() as u32 + 1)..(x1.ceil() as u32)).len();
        } else if x1 < x2 {
            product *= ((x1.floor() as u32 + 1)..(x2.ceil() as u32)).len();
        }
    }

    product
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let re = Regex::new(r"(\d+)( +)").unwrap();
    part1(re.replace_all(input, "$1").as_ref())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 71503);
    }

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";
}