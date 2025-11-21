use num::abs;

#[derive(Copy, Clone, PartialEq)]
struct Galaxy {
    id: usize,
    x: i64,
    y: i64,
    exists: bool
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<Vec<Galaxy>> {
    let mut galaxies = std::iter::repeat_with(Vec::new).take(input.lines().count()).collect::<Vec<Vec<Galaxy>>>();

    let mut id = 1usize;
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            galaxies[i].push(Galaxy {
                id,
                x: j as i64,
                y: i as i64,
                exists: c == '#'
            });
            if c == '#' { id += 1; }
        }
    }

    galaxies
}

fn solve(input: &[Vec<Galaxy>], increase: i64) -> i64 {
    let mut galaxy_map = Vec::from(input);

    for i in 0..galaxy_map.len() {
        if galaxy_map[i].iter().all(|g| !g.exists) {
            galaxy_map
                .iter_mut()
                .skip(i)
                .for_each(|r| r.iter_mut().for_each(|g| g.y += increase));
        }
    }

    for i in 0..input[0].len() {
        if galaxy_map.iter().all(|r| !r[i].exists) {
            galaxy_map
                .iter_mut()
                .for_each(|r| r.iter_mut().skip(i).for_each(|g| g.x += increase));
        }
    }

    let flat_galaxy_map = galaxy_map.iter().flatten().filter(|g| g.exists).collect::<Vec<&Galaxy>>();
    let mut sum = 0i64;
    for (i, g1) in flat_galaxy_map.iter().enumerate() {
        for g2 in flat_galaxy_map.iter().skip(i) {
            sum += abs(g1.x - g2.x) + abs(g1.y - g2.y);
        }
    }
    sum
}

#[aoc(day11, part1)]
fn part1(input: &[Vec<Galaxy>]) -> i64 {
    solve(input, 1)
}

#[aoc(day11, part2)]
fn part2(input: &[Vec<Galaxy>]) -> i64 {
    solve(input, 999_999)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 374);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(&parse(EXAMPLE), 9), 1030);
        assert_eq!(solve(&parse(EXAMPLE), 99), 8410);
    }

    const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
}