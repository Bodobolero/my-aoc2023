#![feature(test)]

extern crate test;
use std::cmp::Ordering;

#[allow(dead_code)] // not yet used in template
const INPUT: &str = include_str!("../inputs/input07.txt");

const STRENGTH: [u8; 37] = [
    //b'A', b'K', b'Q', b'J', b'T', b'9', b'8', b'7', b'6', b'5', b'4', b'3', b'2',
    // digits 0-9
    0, 0, 2, 3, 4, 5, 6, 7, 8, 9, // ascii 3a-40
    0, 0, 0, 0, 0, 0, 0, // A                        J   K                  Q         T
    14, 0, 0, 0, 0, 0, 0, 0, 0, 11, 13, 0, 0, 0, 0, 0, 12, 0, 0, 10,
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn hand_type(hand: &[u8]) -> HandType {
    let mut label_counts = [0u8; 37];
    for card in hand.iter() {
        label_counts[(*card - b'0') as usize] += 1;
    }
    let mut three_of_a_kind = false;
    let mut two_of_a_kind = 0;
    for count in label_counts.iter() {
        if *count == 0 {
            continue;
        }
        if *count == 5 {
            return HandType::FiveOfAKind;
        }
        if *count == 4 {
            return HandType::FourOfAKind;
        }
        if *count == 3 {
            three_of_a_kind = true;
            continue;
        }
        if *count == 2 {
            two_of_a_kind += 1;
            continue;
        }
    }
    if three_of_a_kind {
        if two_of_a_kind > 0 {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind;
    }
    if two_of_a_kind == 2 {
        return HandType::TwoPair;
    }
    if two_of_a_kind == 1 {
        return HandType::OnePair;
    }
    HandType::HighCard
}

fn hand_compare(hand1: &[u8], hand2: &[u8]) -> Ordering {
    for (i, c) in hand1.iter().enumerate() {
        let strength1 = STRENGTH[(c - b'0') as usize];
        let strength2 = STRENGTH[(hand2[i] - b'0') as usize];
        if strength1 == strength2 {
            continue;
        }
        return strength1.cmp(&strength2);
    }
    Ordering::Equal
}

pub fn part1() -> u64 {
    let mut hands: Vec<_> = INPUT
        .lines()
        .map(|line| line.split_ascii_whitespace())
        .map(|mut splits| {
            (
                splits.next().unwrap(),
                splits.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .map(|(hand, count)| (hand, hand_type(hand.as_bytes()), count))
        .collect();
    hands.sort_unstable_by(
        |(hand1, hand_type1, _count1), (hand2, hand_type2, _count2)| {
            if hand_type1 == hand_type2 {
                hand_compare(hand1.as_bytes(), hand2.as_bytes())
            } else {
                hand_type1.cmp(&hand_type2)
            }
        },
    );
    let mut sum: u64 = 0;
    for (i, (_hand, _handtype, count)) in hands.iter().enumerate() {
        //println!("pos {} hand {} count {}", i, hand, count);
        sum += (i + 1) as u64 * count;
    }
    sum
}

const PART2_STRENGTH: [u8; 37] = [
    //b'A', b'K', b'Q', b'J', b'T', b'9', b'8', b'7', b'6', b'5', b'4', b'3', b'2',
    // digits 0-9
    0, 0, 2, 3, 4, 5, 6, 7, 8, 9, // ascii 3a-40
    0, 0, 0, 0, 0, 0, 0, // A                        J   K                  Q         T
    14, 0, 0, 0, 0, 0, 0, 0, 0, 1, 13, 0, 0, 0, 0, 0, 12, 0, 0, 10,
];

fn part2_hand_compare(hand1: &[u8], hand2: &[u8]) -> Ordering {
    for (i, c) in hand1.iter().enumerate() {
        let strength1 = PART2_STRENGTH[(c - b'0') as usize];
        let strength2 = PART2_STRENGTH[(hand2[i] - b'0') as usize];
        if strength1 == strength2 {
            continue;
        }
        return strength1.cmp(&strength2);
    }
    Ordering::Equal
}

fn part2_hand_type(hand: &[u8]) -> HandType {
    let mut label_counts = [0u8; 37];
    let mut jokers = 0;
    for card in hand.iter() {
        if *card == b'J' {
            jokers += 1;
        } else {
            label_counts[(*card - b'0') as usize] += 1;
        }
    }
    if jokers >= 4 {
        return HandType::FiveOfAKind;
    }
    let mut three_of_a_kind = false;
    let mut two_of_a_kind = 0;
    for count in label_counts.iter() {
        if *count + jokers == 5 {
            return HandType::FiveOfAKind;
        }
        if *count + jokers == 4 {
            return HandType::FourOfAKind;
        }
        if *count == 3 {
            three_of_a_kind = true;
            continue;
        }
        if *count == 2 {
            two_of_a_kind += 1;
            continue;
        }
    }
    if three_of_a_kind {
        if two_of_a_kind > 0 {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind;
    }
    if two_of_a_kind == 2 {
        if jokers > 0 {
            return HandType::FullHouse;
        }
        return HandType::TwoPair;
    }
    if two_of_a_kind == 1 {
        if jokers > 0 {
            return HandType::ThreeOfAKind;
        }
        return HandType::OnePair;
    }
    if jokers > 1 {
        return HandType::ThreeOfAKind;
    }
    if jokers > 0 {
        return HandType::OnePair;
    }
    HandType::HighCard
}

pub fn part2() -> u64 {
    let mut hands: Vec<_> = INPUT
        .lines()
        .map(|line| line.split_ascii_whitespace())
        .map(|mut splits| {
            (
                splits.next().unwrap(),
                splits.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .map(|(hand, count)| (hand, part2_hand_type(hand.as_bytes()), count))
        .collect();
    hands.sort_unstable_by(
        |(hand1, hand_type1, _count1), (hand2, hand_type2, _count2)| {
            if hand_type1 == hand_type2 {
                part2_hand_compare(hand1.as_bytes(), hand2.as_bytes())
            } else {
                hand_type1.cmp(&hand_type2)
            }
        },
    );
    let mut sum: u64 = 0;
    for (i, (_hand, _hand_type, count)) in hands.iter().enumerate() {
        // println!(
        //     "pos {} hand {} hand_type {:?} count {}",
        //     i,
        //     hand,
        //     part2_hand_type(hand.as_bytes()),
        //     count
        // );
        sum += (i + 1) as u64 * count;
    }
    sum
}

pub fn main() {
    println!("Part 1: Answer {}", part1());
    println!("Part 2: Answer {} ", part2());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 251106089);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 249620106);
    }
    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        b.iter(part2);
    }
}
