#![feature(test)]

extern crate test;

use regex::Regex;

lazy_static::lazy_static! {
    static ref RE_RULE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
}

const INPUT: &str = include_str!("../inputs/input04.txt");

fn contains(a1: usize, a2: usize, b1: usize, b2: usize) -> bool {
    let i1 = a1..=a2;
    i1.contains(&b1) && i1.contains(&b2)
}

fn overlaps(a1: usize, a2: usize, b1: usize, b2: usize) -> bool {
    let i1 = a1..=a2;
    i1.contains(&b1) || i1.contains(&b2)
}

fn part1() -> usize {
    INPUT
        .lines()
        .filter(|line| {
            let caps = RE_RULE.captures(line).unwrap();
            let (a1, a2, b1, b2) = (
                caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            );
            contains(a1, a2, b1, b2) || contains(b1, b2, a1, a2)
        })
        .count()
}

fn part2() -> usize {
    INPUT
        .lines()
        .filter(|line| {
            let caps = RE_RULE.captures(line).unwrap();
            let (a1, a2, b1, b2) = (
                caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            );
            overlaps(a1, a2, b1, b2) || overlaps(b1, b2, a1, a2)
        })
        .count()
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
        assert_eq!(part1(), 567);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 907);
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
