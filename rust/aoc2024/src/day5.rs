use multimap::MultiMap;

#[aoc_generator(day5)]
fn parse(input: &str) -> (MultiMap<u32, u32>, Vec<Vec<u32>>) {
    let mut rules = MultiMap::new();
    let mut updates = Vec::new();

    let mut after = false;

    for line in input.lines() {
        if line.is_empty() {
            after = true;
            continue;
        }

        if !after {
            let (k, v) = line.split_once('|').unwrap();
            rules.insert(k.parse::<u32>().unwrap(), v.parse::<u32>().unwrap());
        } else {
            updates.push(line.split(',').map(|n| n.parse::<u32>().unwrap()).collect());
        }
    }

    (rules, updates)
}

#[aoc(day5, part1)]
fn part1(input: &(MultiMap<u32, u32>, Vec<Vec<u32>>)) -> u32 {
    input
        .1
        .iter()
        .filter_map(|update| {
            for i in 1..update.len() {
                if let Some(after) = input.0.get_vec(&update[i]) {
                    if update.iter().take(i).any(|n| after.contains(n)) {
                        return None;
                    }
                }
            }

            Some(update[(update.len() - 1) / 2])
        })
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &(MultiMap<u32, u32>, Vec<Vec<u32>>)) -> u32 {
    let input = input.clone();

    input
        .1
        .into_iter()
        .filter_map(|mut update| {
            let mut modified = false;

            while {
                let mut sorted = true;

                for i in 1..update.len() {
                    let num = update[i];

                    if let Some(bad_index) = input.0.get_vec(&num).and_then(|after| {
                        for (j, n) in update.iter().take(i).enumerate() {
                            if after.contains(n) {
                                return Some(j);
                            }
                        }

                        None
                    }) {
                        let bad_num = update.remove(bad_index);
                        update.insert(i, bad_num);

                        modified = true;
                        sorted = false;
                    }
                }

                !sorted
            } {}

            if modified {
                Some(update[(update.len() - 1) / 2])
            } else {
                None
            }
        })
        .sum()
}
