#![feature(test)]

extern crate test;

use rustc_hash::FxHashSet;

const INPUT: &str = include_str!("../inputs/input04.txt");

fn part1() -> usize {
    INPUT
        .lines()
        .map(|line| {
            let mut splits = line.split(": ");
            splits.next();
            let right = splits.next().unwrap();
            let mut rsplits = right.split(" | ");
            let win_str = rsplits.next().unwrap();
            let have_str = rsplits.next().unwrap();
            let winning_numbers: FxHashSet<u32> = win_str
                .split_ascii_whitespace()
                .map(|nrstr| nrstr.parse::<u32>().unwrap())
                .collect();
            // println!("winning numbers: {:?}", winning_numbers);
            let winners = have_str
                .split_ascii_whitespace()
                .map(|nrstr| nrstr.parse::<u32>().unwrap())
                .filter(|nr| winning_numbers.contains(nr))
                .count();
            // println!("have_str {} has {} winning numbers", have_str, winners);
            if winners > 0 {
                2usize.pow((winners - 1) as u32)
            } else {
                0
            }
        })
        .sum::<usize>()
}

fn part2() -> usize {
    // create an array of matching numbers
    let mut matched_numbers: Vec<usize> = Vec::new();
    let _: Vec<_> = INPUT
        .lines()
        .map(|line| {
            let mut splits = line.split(": ");
            splits.next();
            let right = splits.next().unwrap();
            let mut rsplits = right.split(" | ");
            let win_str = rsplits.next().unwrap();
            let have_str = rsplits.next().unwrap();
            let winning_numbers: FxHashSet<u32> = win_str
                .split_ascii_whitespace()
                .map(|nrstr| nrstr.parse::<u32>().unwrap())
                .collect();
            // println!("winning numbers: {:?}", winning_numbers);
            let winners = have_str
                .split_ascii_whitespace()
                .map(|nrstr| nrstr.parse::<u32>().unwrap())
                .filter(|nr| winning_numbers.contains(nr))
                .count();
            matched_numbers.push(winners);
        })
        .collect();
    println!("matching numbers per card: {:?}", matched_numbers);
    let mut card_counts = vec![1usize; matched_numbers.len()];
    for (i, matched) in matched_numbers.into_iter().enumerate() {
        let num_cards = card_counts[i]; // number of cards at position i
        let wins = std::cmp::min(matched, card_counts.len() - i - 1);
        for j in i + 1..i + wins + 1 {
            card_counts[j] += num_cards;
        }
    }
    card_counts.into_iter().sum()
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
        assert_eq!(part1(), 23678);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 15455663);
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
