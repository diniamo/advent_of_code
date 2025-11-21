use aoc2022::Day;


pub struct Day2 {}

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSIORS: i32 = 3;

const LOSE: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;
impl Day for Day2 {
    fn day_number(&self) -> u8 {
        2
    }

    /*
    A, X, 1 -> Rock
    B, Y, 2 -> Paper
    C, Z, 3 -> Scissiors

    Lose -> 0
    Draw -> 3
    Win -> 6
    */
    fn part1(&self, input: &String) -> String {
        let input_pairs = input.trim().split('\n').map(|line| {
            let pair = line.split_once(' ').unwrap();
            (pair.0.chars().nth(0).unwrap(), pair.1.chars().nth(0).unwrap())
        }).collect::<Vec<(char, char)>>();

        let mut points = 0;
        for pair in input_pairs {
            match pair.0 {
                'A' => {
                    match pair.1 {
                        'X' => points += ROCK + DRAW,
                        'Y' => points += PAPER + WIN,
                        'Z' => points += SCISSIORS + LOSE,
                        _ => unreachable!()
                    }
                },
                'B' => {
                    match pair.1 {
                        'X' => points += ROCK + LOSE,
                        'Y' => points += PAPER + DRAW,
                        'Z' => points += SCISSIORS + WIN,
                        _ => unreachable!()
                    }
                },
                'C' => {
                    match pair.1 {
                        'X' => points += ROCK + WIN,
                        'Y' => points += PAPER + LOSE,
                        'Z' => points += SCISSIORS + DRAW,
                        _ => unreachable!()
                    }
                },
                _ => unreachable!()
            }
        }

        points.to_string()
    }


    /*
    A, 1 -> Rock
    B, 2 -> Paper
    C, 3 -> Scissiors

    X -> lose
    Y -> draw
    Z -> win

    Lose -> 0
    Draw -> 3
    Win -> 6
    */
    fn part2(&self, input: &String) -> String {
        let input_pairs = input.trim().split('\n').map(|line| {
            let pair = line.split_once(' ').unwrap();
            (pair.0.chars().nth(0).unwrap(), pair.1.chars().nth(0).unwrap())
        }).collect::<Vec<(char, char)>>();

        let mut points = 0;
        for pair in input_pairs {
            match pair.0 {
                'A' => {
                    match pair.1 {
                        'X' => points += SCISSIORS + LOSE,
                        'Y' => points += ROCK + DRAW,
                        'Z' => points += PAPER + WIN,
                        _ => unreachable!()
                    }
                },
                'B' => {
                    match pair.1 {
                        'X' => points += ROCK + LOSE,
                        'Y' => points += PAPER + DRAW,
                        'Z' => points += SCISSIORS + WIN,
                        _ => unreachable!()
                    }
                },
                'C' => {
                    match pair.1 {
                        'X' => points += PAPER + LOSE,
                        'Y' => points += SCISSIORS + DRAW,
                        'Z' => points += ROCK + WIN,
                        _ => unreachable!()
                    }
                },
                _ => unreachable!()
            }
        }

        points.to_string()
    }
}
