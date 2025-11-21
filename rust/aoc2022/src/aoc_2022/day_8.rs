use aoc2022::Day;

use std::cmp::max;


pub struct Day8 {}

impl Day for Day8 {
    fn day_number(&self) -> u8 {
        8
    }

    fn part1(&self, input: &String) -> String {
        let map: Vec<Vec<u32>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

        // map.iter().map(|row| row.iter().map(|digit| digit.to_string()).collect()).map(|row: Vec<String>| row.join(" ")).for_each(|line| println!("{line}"));

        // The initial value here is the perimeter of the grid, as we don't account for that in the loop
        let mut visible: usize = 2*(map.len() - 2 + map[0].len() - 2) + 4;
        for (i, row) in (&map[1..(map.len() - 1)]).iter().enumerate() {
            for (j, &digit) in (&row[1..(row.len() - 1)]).iter().enumerate() {

                // Left, right, up, down
                if (&row[..=j]).iter().all(|&d| d < digit)    // Wouldn't be inclusive, but we start from row[1] and j is still 0
                    || (&row[(j + 2)..]).iter().all(|&d| d < digit)    // Would be j+1, reason above
                    || (&map[..=i]).iter().all(|r| r[j + 1] < digit)    // Wouldn't be inclusive, but we start from map[1] and i is still 0
                    || (&map[(i + 2)..]).iter().all(|r| r[j + 1] < digit) {    // Would be i+1 and j, reason(s) above
                    visible += 1;
                }
                
            }
        }
        
        visible.to_string()
    }

    fn part2(&self, input: &String) -> String {
        let map: Vec<Vec<u32>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

        // map.iter().map(|row| row.iter().map(|digit| digit.to_string()).collect()).map(|row: Vec<String>| row.join(" ")).for_each(|line| println!("{line}"));

        let mut highest: usize = 0;
        for (i, row) in (&map[1..(map.len() - 1)]).iter().enumerate() {
            for (j, &digit) in (&row[1..(row.len() - 1)]).iter().enumerate() {
                let left_side = &row[..=j];
                let left = left_side.iter().rev().enumerate().find_map(|(k, &d)| if digit <= d || k == (left_side.len() - 1) { Some(k + 1) } else { None }).unwrap();
                let right_side = &row[(j + 2)..];
                let right = right_side.iter().enumerate().find_map(|(k, &d)| if digit <= d || k == (right_side.len() - 1) { Some(k + 1) } else { None }).unwrap();
                let top_side = &map[..=i];
                let up = top_side.iter().rev().enumerate().find_map(|(k, r)| if digit <= r[j + 1] || k == (top_side.len() - 1) { Some(k + 1) } else { None }).unwrap();
                let bottom_side = &map[(i + 2)..];
                let down = bottom_side.iter().enumerate().find_map(|(k, r)| if digit <= r[j + 1] || k == (bottom_side.len() - 1) { Some(k + 1) } else { None }).unwrap();

                highest = max(highest, left * right * up * down);
            }
        }

        highest.to_string()
    }
}
