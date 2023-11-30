#![feature(test)]

extern crate test;
const INPUT: &str = include_str!("../inputs/input22.txt");

fn part1() -> usize {
    0
}

fn part2() -> usize {
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
        assert_eq!(part1(), 0);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 0);
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
