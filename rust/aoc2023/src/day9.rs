use itertools::Itertools;

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn get_diff_row(row: &[i32]) -> Vec<i32> {
    let mut row_builder: Vec<i32> = Vec::new();
    for (i, number) in row.iter().enumerate() {
        if i == 0 { continue; }

        row_builder.push(number - row[i - 1]);
    }
    row_builder
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|seq| {
        let mut diff_rows: Vec<Vec<i32>> = vec![seq.clone()];
        while !diff_rows.last().unwrap().iter().all_equal() {
            diff_rows.push(get_diff_row(diff_rows.last().unwrap()));
        }

        // This is the intuitive solution, but we're just adding the last number in each diff_row together
        // diff_rows.reverse();
        // let mut i = 1usize;
        // while i < diff_rows.len() {
        //     let previous = *diff_rows[i - 1].last().unwrap();
        //     let row = &mut diff_rows[i];
        //     row.push(row.last().unwrap() + previous);
        //
        //     i += 1;
        // }
        // sum += diff_rows.last().unwrap().last().unwrap();

        diff_rows
            .iter()
            .map(|r| r.last().unwrap())
            .sum::<i32>()
    }).sum()
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<i32>]) -> i32 {
    part1(
        &input
            .iter()
            .map(|s| s.iter().copied().rev().collect())
            .collect::<Vec<Vec<i32>>>()
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 2);
    }

    #[test]
    fn diff_row() {
        let rows = parse(EXAMPLE);
        assert_eq!(get_diff_row(rows.last().unwrap()), vec![3, 3, 5, 9, 15])
    }

    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
}