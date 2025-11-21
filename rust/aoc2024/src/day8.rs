use std::collections::HashSet;

struct Input {
    rows: i32,
    cols: i32,
    antennas: Vec<(char, i32, i32)>,
}

fn check_bounds(row: i32, col: i32, rows: i32, cols: i32) -> bool {
    row >= 0 && row < rows && col >= 0 && col < cols
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    let mut antennas = Vec::new();

    for (row, line) in input.lines().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            if chr != '.' {
                antennas.push((chr, row as i32, col as i32));
            }
        }
    }

    Input {
        rows: input.lines().count() as i32,
        cols: input.lines().next().unwrap().chars().count() as i32,
        antennas,
    }
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    let mut locations = HashSet::new();

    for (i, (chr, row, col)) in input.antennas.iter().enumerate() {
        for (ch, r, c) in input.antennas.iter().skip(i) {
            if chr != ch || (row == r && col == c) { continue; }

            let row_diff = r - row;
            let col_diff = c - col;

            let row1 = row - row_diff;
            let col1 = col - col_diff;
            if check_bounds(row1, col1, input.rows, input.cols) {
                locations.insert((row1, col1));
            }

            let row2 = r + row_diff;
            let col2 = c + col_diff;
            if check_bounds(row2, col2, input.rows, input.cols) {
                locations.insert((row2, col2));
            }
        }
    }

    locations.len()
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> usize {
    let mut locations = HashSet::new();

    for (i, (chr, row, col)) in input.antennas.iter().enumerate() {
        for (ch, r, c) in input.antennas.iter().skip(i) {
            if chr != ch || (row == r && col == c) { continue; }

            locations.insert((*row, *col));
            locations.insert((*r, *c));

            let row_diff = r - row;
            let col_diff = c - col;

            let mut row_node = row - row_diff;
            let mut col_node = col - col_diff;
            while check_bounds(row_node, col_node, input.rows, input.cols) {
                locations.insert((row_node, col_node));

                row_node -= row_diff;
                col_node -= col_diff;
            }

            row_node = r + row_diff;
            col_node = c + col_diff;
            while check_bounds(row_node, col_node, input.rows, input.cols) {
                locations.insert((row_node, col_node));

                row_node += row_diff;
                col_node += col_diff;
            }
        }
    }

    locations.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 34);
    }
}
