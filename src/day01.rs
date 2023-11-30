#![feature(test)]

use itertools::Itertools;

extern crate test;
const INPUT: &str = include_str!("../inputs/input01.txt");

fn part1() -> u32 {
    INPUT
        .split("\n\n")
        .map(|lines| lines.lines().map(|n| n.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
}

fn part2() -> u32 {
    INPUT
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|n| n.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        // need to sort descending because we want top three
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
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
        assert_eq!(part1(), 68442);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 204837);
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
