use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position {
    row: isize,
    col: isize,
    row_step: isize,
    col_step: isize,
}

struct Map {
    tiles: Vec<Vec<char>>,
    start_pos: Position,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Map {
    let mut map = Map {
        tiles: Vec::new(),
        start_pos: Position {
            row: 0,
            col: 0,
            row_step: -1,
            col_step: 0,
        },
    };

    for (r, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (c, chr) in line.chars().enumerate() {
            if chr == '^' {
                map.start_pos.row = r as isize;
                map.start_pos.col = c as isize;
            }

            row.push(chr);
        }

        map.tiles.push(row);
    }

    map
}

fn walk<T: FnMut(&Position) -> bool>(map: &Map, mut callback: T) {
    let mut pos = map.start_pos.clone();

    loop {
        let mut row_new_i = pos.row + pos.row_step;
        let mut col_new_i = pos.col + pos.col_step;

        if row_new_i < 0 || col_new_i < 0 {
            callback(&pos);
            break;
        }

        let mut row_new_u = row_new_i as usize;
        let mut col_new_u = col_new_i as usize;

        if row_new_u >= map.tiles.len() || col_new_u >= map.tiles[row_new_u].len() {
            callback(&pos);
            break;
        }

        while map.tiles[row_new_u][col_new_u] == '#' {
            if pos.col_step == 0 {
                pos.col_step = -pos.row_step;
                pos.row_step = 0;
            } else {
                pos.row_step = pos.col_step;
                pos.col_step = 0;
            }

            row_new_i = pos.row + pos.row_step;
            col_new_i = pos.col + pos.col_step;

            row_new_u = row_new_i as usize;
            col_new_u = col_new_i as usize;
        }

        if !callback(&pos) {
            break;
        }

        pos.row = row_new_i;
        pos.col = col_new_i;
    }
}

#[aoc(day6, part1)]
fn part1(input: &Map) -> usize {
    let mut positions: HashSet<(isize, isize)> = HashSet::new();

    walk(input, |pos| {
        positions.insert((pos.row, pos.col));
        true
    });

    positions.len()
}

#[aoc(day6, part2)]
fn part2(input: &Map) -> usize {
    let mut count = 0usize;

    let mut checked = HashSet::new();
    checked.insert((input.start_pos.row, input.start_pos.col));
    let mut prev_pos = input.start_pos.clone();

    walk(input, |pos| {
        if !checked.contains(&(pos.row, pos.col)) {
            let mut map = Map {
                tiles: input.tiles.clone(),
                start_pos: Position {
                    row: prev_pos.row,
                    col: prev_pos.col,
                    row_step: prev_pos.row_step,
                    col_step: prev_pos.col_step,
                },
            };
            map.tiles[pos.row as usize][pos.col as usize] = '#';

            let mut positions = HashSet::new();
            let mut is_first = true;

            walk(&map, |p| {
                if !is_first && positions.contains(p) {
                    count += 1;
                    false
                } else {
                    positions.insert(p.clone());
                    is_first = false;
                    true
                }
            });
        }

        checked.insert((pos.row, pos.col));
        prev_pos = pos.clone();

        true
    });

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 6);
    }
}
