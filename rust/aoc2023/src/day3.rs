fn is_symbol(character: char) -> bool {
    character != '.' && !character.is_ascii_digit()
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<char>]) -> u32 {
    let mut sum = 0u32;

    let mut number_builder = String::new();
    let mut symbol_found = false;
    for (i, chars) in input.iter().enumerate() {
        for (j, char) in chars.iter().enumerate() {
            if char.is_ascii_digit() {
                number_builder.push(*char);

                if !symbol_found {
                    let check_left = j >= 1;
                    let check_right = j + 1 < chars.len();
                    let check_above = i >= 1;
                    let check_below = i + 1 < chars.len();

                    // Sides
                    if check_left && is_symbol(chars[j - 1]) {
                        symbol_found = true;
                        continue;
                    }
                    if check_right && is_symbol(chars[j + 1]) {
                        symbol_found = true;
                        continue;
                    }

                    // Row above
                    if check_above {
                        let row_above = &input[i - 1];

                        if check_left && is_symbol(row_above[j - 1])  { symbol_found = true; continue; }
                        if is_symbol(row_above[j])                    { symbol_found = true; continue; }
                        if check_right && is_symbol(row_above[j + 1]) { symbol_found = true; continue; }
                    }

                    // Row below
                    if check_below {
                        let row_below = &input[i + 1];

                        if check_left && is_symbol(row_below[j - 1])  { symbol_found = true; continue; }
                        if is_symbol(row_below[j])                    { symbol_found = true; continue; }
                        if check_right && is_symbol(row_below[j + 1]) { symbol_found = true; continue; }
                    }
                }
            } else {
                if symbol_found {
                    sum += number_builder.parse::<u32>().unwrap();
                    symbol_found = false;
                }
                number_builder.clear();
            }
        }
    }

    sum
}

// This function assumes that line[index] is a digit
fn get_full_number(index: usize, line: &[char]) -> u32 {
    let mut number_builder = String::from(line[index]);

    let mut is_checking_left = true;
    let mut is_checking_right = true;
    let mut i = 1;
    loop {
        // Left
        if is_checking_left && index >= i && line[index - i].is_ascii_digit() {
            number_builder.insert(0, line[index - i]);
        } else {
            is_checking_left = false;
        }

        // Right
        if is_checking_right && index + i < line.len() && line[index + i].is_ascii_digit() {
            number_builder.push(line[index + i]);
        } else {
            is_checking_right = false;
        }

        if !is_checking_left && !is_checking_right {
            return number_builder.parse().unwrap();
        }

        i += 1;
    }
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<char>]) -> u32 {
    let mut sum = 0u32;

    for (i, chars) in input.iter().enumerate() {
        for (j, char) in chars.iter().enumerate() {
            if *char == '*' {
                let mut product = 1u32;
                let mut count = 0u8;

                let check_left = j >= 1;
                let check_right = j + 1 < chars.len();
                let check_above = i >= 1;
                let check_below = i + 1 < chars.len();

                // Sides
                if check_left && chars[j - 1].is_ascii_digit() {
                    product *= get_full_number(j - 1, chars);
                    count += 1;
                }
                if check_right && chars[j + 1].is_ascii_digit() {
                    product *= get_full_number(j + 1, chars);
                    count += 1;
                }

                // Row above
                if check_above {
                    let row_above = &input[i - 1];

                    if check_left && row_above[j - 1].is_ascii_digit() {
                        product *= get_full_number(j - 1, row_above);
                        count += 1;

                        if !row_above[j].is_ascii_digit() && check_right && row_above[j + 1].is_ascii_digit() {
                            product *= get_full_number(j + 1, row_above);
                            count += 1;
                        }
                    } else if row_above[j].is_ascii_digit() {
                        product *= get_full_number(j, row_above);
                        count += 1;
                    } else if check_right && row_above[j + 1].is_ascii_digit() {
                        product *= get_full_number(j + 1, row_above);
                        count += 1;
                    }
                }

                // Row below
                if check_below {
                    let row_below = &input[i + 1];

                    if check_left && row_below[j - 1].is_ascii_digit() {
                        product *= get_full_number(j - 1, row_below);
                        count += 1;

                        if !row_below[j].is_ascii_digit() && check_right && row_below[j + 1].is_ascii_digit() {
                            product *= get_full_number(j + 1, row_below);
                            count += 1;
                        }
                    } else if row_below[j].is_ascii_digit() {
                        product *= get_full_number(j, row_below);
                        count += 1;
                    } else if check_right && row_below[j + 1].is_ascii_digit() {
                        product *= get_full_number(j + 1, row_below);
                        count += 1;
                    }
                }

                if count == 2 {
                    sum += product;
                }
            }
        }
    }

    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symbol() {
        assert!(is_symbol('%'));
        assert!(!is_symbol('.'));
        assert!(!is_symbol('5'));
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..")), 4361);
    }

    #[test]
    fn test_get_full_number() {
        assert_eq!(get_full_number(3, &['.', '.', '.', '4', '2', '0']), 420);
        assert_eq!(get_full_number(3, &['.', '4', '2', '0', '.', '.']), 420);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..")), 467835);
    }

    #[test]
    fn part2_example_dalv() {
        assert_eq!(part2(&parse("12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56")), 6756);
    }
}