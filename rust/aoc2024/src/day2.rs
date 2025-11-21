enum Type {
    Decreasing,
    Increasing,
}

impl Type {
    fn from_report(report: &[i32]) -> Option<Self> {
        for i in 1..report.len() {
            let diff = report[i] - report[i - 1];

            if diff.is_negative() {
                return Some(Self::Decreasing);
            } else if diff.is_positive() {
                return Some(Self::Increasing);
            }
        }

        None
    }

    fn validate_diff(&self, diff: i32) -> bool {
        (1..=3).contains(&diff.abs())
            && match self {
                Type::Decreasing => diff.is_negative(),
                Type::Increasing => diff.is_positive(),
            }
    }
}

#[derive(PartialEq, Eq)]
enum ValidationResult {
    Valid,
    Invalid(usize),
    NonRecoverable,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn validate_report(report: &[i32]) -> ValidationResult {
    let typ = if let Some(typ) = Type::from_report(report) {
        typ
    } else {
        return ValidationResult::NonRecoverable;
    };

    for i in 1..report.len() {
        if !typ.validate_diff(report[i] - report[i - 1]) {
            return ValidationResult::Invalid(i);
        }
    }

    ValidationResult::Valid
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<i32>]) -> usize {
    input
        .iter()
        .filter(|report| validate_report(report) == ValidationResult::Valid)
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<i32>]) -> usize {
    input
        .iter()
        .filter(|report| match validate_report(report) {
            ValidationResult::Valid => true,
            ValidationResult::NonRecoverable => false,
            ValidationResult::Invalid(index) => {
                let mut dampened = report.to_vec();
                dampened.remove(index - 1);
                if validate_report(&dampened) == ValidationResult::Valid {
                    return true;
                }

                dampened = report.to_vec();
                dampened.remove(index);
                if validate_report(&dampened) == ValidationResult::Valid {
                    return true;
                }

                dampened = report.to_vec();
                dampened.remove(0);
                if validate_report(&dampened) == ValidationResult::Valid {
                    return true;
                }

                false
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part2_example() {
        assert_eq!(part2(parse(EXAMPLE).as_ref()), 4);
    }
}
