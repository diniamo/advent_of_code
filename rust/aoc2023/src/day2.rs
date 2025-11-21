struct Game {
    id: usize,
    red: u32,
    green: u32,
    blue: u32
}

enum Draw {
    Red(u32),
    Green(u32),
    Blue(u32)
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|l| (
            l.0.split_once(' ').unwrap().1.parse::<usize>().unwrap(),
            l.1.split("; ").map(|draws| draws.split(", ").map(|draw| {
                let split = draw.split_once(' ').unwrap();
                let count = split.0.parse::<u32>().unwrap();

                match split.1 {
                    "red" => Draw::Red(count),
                    "green" => Draw::Green(count),
                    "blue" => Draw::Blue(count),
                    _ => unreachable!()
                }
            }))
        ))
        .map(|parsed| {
            let mut game = Game {
                id: parsed.0,
                red: 0,
                green: 0,
                blue: 0
            };

            for draws in parsed.1 {
                for draw in draws {
                    match draw {
                        Draw::Red(c) => if c > game.red { game.red = c },
                        Draw::Green(c) => if c > game.green { game.green = c },
                        Draw::Blue(c) => if c > game.blue { game.blue = c },
                    }
                }
            }

            game
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> usize {
    let max = Game {
        id: 0,
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .iter()
        .filter(|g| g.red <= max.red && g.green <= max.green && g.blue <= max.blue)
        .map(|g| g.id)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|g| g.red * g.green * g.blue)
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")), 2286);
    }
}