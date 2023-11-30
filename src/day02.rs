#![feature(test)]

extern crate test;

use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/input02.txt");

fn part1() -> u32 {
    let score_rules: HashMap<&str, u32> = [
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]
    .into_iter()
    .collect();
    INPUT
        .lines()
        .map(|game| score_rules.get(game).unwrap())
        .sum()
}

fn part2() -> u32 {
    let score_rules: HashMap<&str, u32> = [
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]
    .into_iter()
    .collect();
    INPUT
        .lines()
        .map(|game| score_rules.get(game).unwrap())
        .sum()
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
        assert_eq!(part1(), 10310);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 14859);
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
