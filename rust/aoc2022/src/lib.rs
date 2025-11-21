pub trait Year {
    fn year_number(&self) -> u16;

    fn days(&self) -> Vec<Box<dyn Day>>;
}

pub trait Day {
    fn day_number(&self) -> u8;

    fn part1(&self, input: &String) -> String;
    fn part2(&self, input: &String) -> String;
}
