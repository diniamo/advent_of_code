use std::{fs::{create_dir_all, self}, path::Path};

use aoc2022::Year;
use crate::aoc_2022::*;

mod aoc_2022;

fn main() -> std::io::Result<()> {
    let session_id = fs::read_to_string("./session_id.txt")?;

    let years = [
        Aoc2022 {} 
    ];

    for year in years {
        let year_num = year.year_number();
        println!("Running year {}:", year_num);

        create_dir_all(format!("./input/{}", year_num))?;

        for day in year.days() {
            let input_file = format!("./input/{}/{}.txt", year_num, day.day_number());
            let input = if Path::new(&input_file).exists() {
                fs::read_to_string(input_file)?
            } else {
                let request = minreq::get(format!("https://adventofcode.com/{}/day/{}/input", year_num, day.day_number()))
                    .with_header("Cookie", format!("session={}", session_id));
                let response = request.send().unwrap();
                let body = response.as_str().unwrap();

                fs::write(input_file, body)?;
                String::from(body)
            };

            println!("  Day {}\n    Part 1: {}\n    Part 2: {}", day.day_number(), day.part1(&input), day.part2(&input));
        }
    }

    Ok(())
}
