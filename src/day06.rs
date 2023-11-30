#![feature(test)]

extern crate test;

use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/input06.txt");

fn part1() -> usize {
    for i in 0..INPUT.as_bytes().len() - 3 {
        let mut set: HashSet<u8> = HashSet::new();
        for j in 0..4 {
            if !set.insert(INPUT.as_bytes()[i + j]) {
                break;
            }
        }
        if set.len() == 4 {
            return i + 4;
        }
    }
    0
}

fn part2() -> usize {
    for i in 0..INPUT.as_bytes().len() - 3 {
        let mut set: HashSet<u8> = HashSet::new();
        for j in 0..14 {
            if !set.insert(INPUT.as_bytes()[i + j]) {
                break;
            }
        }
        if set.len() == 14 {
            return i + 14;
        }
    }
    0
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
        assert_eq!(part1(), 1262);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 3444);
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
