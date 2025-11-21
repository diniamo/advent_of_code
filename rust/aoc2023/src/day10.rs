use std::cmp::Ordering;
use std::mem::swap;

#[derive(Copy, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize
}

impl Position {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0
        }
    }
}

#[derive(Copy, Clone)]
struct Cursor {
    current: Position,
    previous: Position
}

impl Cursor {
    fn new() -> Self {
        Self {
            current: Position::new(),
            previous: Position::new()
        }
    }

    fn set(&mut self, x: usize, y: usize) {
        swap(&mut self.current, &mut self.previous);
        self.current.x = x;
        self.current.y = y;
    }
}

impl PartialEq<Self> for Cursor {
    fn eq(&self, other: &Self) -> bool {
        self.current.x == other.current.x && self.current.y == other.current.y
    }
}
impl Eq for Cursor {}

impl PartialEq<Position> for Cursor {
    fn eq(&self, other: &Position) -> bool {
        self.current.x == other.x && self.current.y == other.y
    }
}

enum Direction {
    North,
    East,
    South,
    West
}

impl From<Cursor> for Direction {
    fn from(value: Cursor) -> Self {
        let Position { x: cx, y: cy } = value.current;
        let Position { x: px, y: py } = value.previous;

        match (cx.cmp(&px), cy.cmp(&py)) {
            (Ordering::Equal, Ordering::Greater) => Direction::South,
            (Ordering::Equal, Ordering::Less) => Direction::North,
            (Ordering::Greater, _) => Direction::East,
            (Ordering::Less, _) => Direction::West,
            _ => unreachable!()
        }
    }
}

enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartingPosition
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::StartingPosition,
            _ => unreachable!()
        }
    }
}

impl Pipe {
    fn north(&self) -> bool { matches!(self, Self::Vertical | Self::NorthWest | Self::NorthEast) }
    fn east(&self) -> bool { matches!(self, Self::Horizontal | Self::NorthEast | Self::SouthEast) }
    fn south(&self) -> bool { matches!(self, Self::Vertical | Self::SouthEast | Self::SouthWest) }
    fn west(&self) -> bool { matches!(self, Self::Horizontal | Self::SouthWest | Self::NorthWest) }

    fn apply(&self, cursor: &mut Cursor) {
        match Direction::from(*cursor) {
            Direction::North => {
                if self.north() {
                    cursor.set(cursor.current.x, cursor.current.y - 1);
                } else if self.east() {
                    cursor.set(cursor.current.x + 1, cursor.current.y);
                } else if self.west() {
                    cursor.set(cursor.current.x - 1, cursor.current.y);
                }
            }
            Direction::East => {
                if self.north() {
                    cursor.set(cursor.current.x, cursor.current.y - 1);
                } else if self.east() {
                    cursor.set(cursor.current.x + 1, cursor.current.y);
                } else if self.south() {
                    cursor.set(cursor.current.x, cursor.current.y + 1);
                }
            }
            Direction::South => {
                if self.west() {
                    cursor.set(cursor.current.x - 1, cursor.current.y);
                } else if self.east() {
                    cursor.set(cursor.current.x + 1, cursor.current.y);
                } else if self.south() {
                    cursor.set(cursor.current.x, cursor.current.y + 1);
                }
            }
            Direction::West => {
                if self.west() {
                    cursor.set(cursor.current.x - 1, cursor.current.y);
                } else if self.north() {
                    cursor.set(cursor.current.x, cursor.current.y - 1);
                } else if self.south() {
                    cursor.set(cursor.current.x, cursor.current.y + 1);
                }
            }
        }
    }
}

impl From<&Pipe> for char {
    fn from(value: &Pipe) -> Self {
        match value {
            Pipe::Vertical => '|',
            Pipe::Horizontal => '-',
            Pipe::NorthEast => 'L',
            Pipe::NorthWest => 'J',
            Pipe::SouthWest => '7',
            Pipe::SouthEast => 'F',
            Pipe::Ground => '.',
            Pipe::StartingPosition => 'S',
        }
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Vec<Pipe>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.into()).collect())
        .collect()
}

fn get_starting_cursors(lines: &[Vec<Pipe>]) -> (Position, Cursor, Cursor) {
    let mut cursor_1 = Cursor::new();
    let mut cursor_2 = Cursor::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if matches!(*c, Pipe::StartingPosition) {
                let start = Position { x, y };
                cursor_1.set(x, y);
                cursor_2.set(x, y);

                'block: {
                    if lines[y - 1][x].south() {
                        if cursor_1 == start { cursor_1.set(x, y - 1); } else {
                            cursor_2.set(x, y - 1);
                            break 'block;
                        }
                    }
                    if lines[y][x + 1].west() {
                        if cursor_1 == start { cursor_1.set(x + 1, y); } else {
                            cursor_2.set(x + 1, y);
                            break 'block;
                        }
                    }
                    if lines[y + 1][x].north() {
                        if cursor_1 == start { cursor_1.set(x, y + 1); } else {
                            cursor_2.set(x, y + 1);
                            break 'block;
                        }
                    }
                    if lines[y][x - 1].east() {
                        if cursor_1 == start { cursor_1.set(x - 1, y); } else {
                            cursor_2.set(x - 1, y);
                            break 'block;
                        }
                    }
                }

                return (start, cursor_1, cursor_2);
            }
        }
    }

    unreachable!();
}

#[aoc(day10, part1)]
fn part1(lines: &[Vec<Pipe>]) -> u32 {
    let (_, mut cursor_1, mut cursor_2) = get_starting_cursors(lines);

    let mut steps = 1u32;

    while cursor_1 != cursor_2 {
        let pipe_1 = &lines[cursor_1.current.y][cursor_1.current.x];
        let pipe_2 = &lines[cursor_2.current.y][cursor_2.current.x];

        pipe_1.apply(&mut cursor_1);
        pipe_2.apply(&mut cursor_2);

        steps += 1;
    }

    steps
}

#[aoc(day10, part2)]
fn part2(lines: &[Vec<Pipe>]) -> u32 {
    let mut pipes: Vec<Position> = Vec::new();

    let (start, mut cursor, _) = get_starting_cursors(lines);
    pipes.push(start);

    while cursor != start {
        let pipe = &lines[cursor.current.y][cursor.current.x];
        pipes.push(Position {
            x: cursor.current.x,
            y: cursor.current.y
        });
        pipe.apply(&mut cursor);
    }

    let mut sum = 0u32;
    let mut in_pipe = false;
    for (y, l) in lines.iter().enumerate() {
        for (x, p) in l.iter().enumerate() {
            if pipes.iter().any(|p| p.x == x && p.y == y) {
                if p.north() { in_pipe = !in_pipe; }
                continue;
            }

            if in_pipe {
                sum += 1;
            }
        }

        in_pipe = false;
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 4);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_2)), 4);
        assert_eq!(part2(&parse(EXAMPLE_3)), 8);
    }

    const EXAMPLE_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const EXAMPLE_2: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    pub const EXAMPLE_3: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
}