use std::cmp::Ordering;
use itertools::Itertools;

const CARDS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
struct Hand {
    cards: [char; 5],
    hand_type: HandType,
    bid: u32,
    j_is_joker: bool
}

enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn rank(&self) -> u8 {
        match self {
            Self::FiveOfAKind => 7,
            Self::FourOfAKind => 6,
            Self::FullHouse => 5,
            Self::ThreeOfAKind => 4,
            Self::TwoPair => 3,
            Self::OnePair => 2,
            Self::HighCard => 1
        }
    }
}

impl From<(bool, &str)> for Hand {
    fn from(value: (bool, &str)) -> Self {
        let j_is_joker = value.0;
        let (hand_str, bid) = value.1.split_once(' ').unwrap();
        let bid = bid.parse::<u32>().unwrap();
        let hand: [char; 5] = hand_str.chars().collect::<Vec<char>>().try_into().unwrap();

        let mut same = hand
            .iter()
            .filter(|c| {
                if j_is_joker {
                    **c != 'J'
                } else {
                    true
                }
            })
            .unique()
            .map(|card| hand.iter().filter(|c| **c == *card).count())
            .collect::<Vec<usize>>();
        same.sort_by(|a, b| b.cmp(a));
        if j_is_joker {
            if same.is_empty() {
                same.push(5);
            } else {
                same[0] += hand.iter().filter(|c| **c == 'J').count();
            }
        }

        Hand {
            hand_type: if same[0] == 5 {
                HandType::FiveOfAKind
            } else if same[0] == 4 {
                HandType::FourOfAKind
            } else if same[0] == 3 && same[1] == 2 {
                HandType::FullHouse
            } else if same[0] == 3 {
                HandType::ThreeOfAKind
            } else if same[0] == 2 && same[1] == 2 {
                HandType::TwoPair
            } else if same[0] == 2 {
                HandType::OnePair
            } else {
                HandType::HighCard
            },
            cards: hand,
            bid,
            j_is_joker
        }
    }
}

impl Eq for Hand {}
impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_rank = self.hand_type.rank();
        let other_rank = other.hand_type.rank();
        if self_rank != other_rank {
            return self_rank.cmp(&other_rank);
        }

        if self.cards != other.cards {
            for (i, card) in self.cards.iter().enumerate() {
                let self_index = if self.j_is_joker && *card == 'J' { 0 } else { CARDS.iter().position(|c| *c == *card).unwrap() + 1 };
                let other_index = if other.j_is_joker && other.cards[i] == 'J' { 0 } else { CARDS.iter().position(|c| *c == other.cards[i]).unwrap() + 1 };

                if self_index != other_index {
                    return self_index.cmp(&other_index);
                }
            }
        }

        Ordering::Equal
    }
}

fn get_ranked_sum(hands: &[Hand]) -> u32 {
    let mut sum = 0u32;
    for (i, hand) in hands.iter().enumerate() {
        sum += ((i as u32) + 1) * hand.bid;
    }
    sum
}

#[aoc(day7, part1)]
fn part1(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|l| (false, l).into())
        .collect::<Vec<Hand>>();
    hands.sort();
    get_ranked_sum(&hands)
}

#[aoc(day7, part2)]
fn part2(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|l| (true, l).into())
        .collect::<Vec<Hand>>();
    hands.sort();
    get_ranked_sum(&hands)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_2), 6592);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_2), 6839);
    }

    const _EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    const EXAMPLE_2: &str = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";
}