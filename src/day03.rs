#![feature(test)]

use itertools::Itertools;

extern crate test;

const INPUT: &str = include_str!("../inputs/input03.txt");

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn codechar(c: u8) -> u32 {
    if (b'a'..=b'z').contains(&c) {
        (c - b'a' + 1) as u32
    } else {
        (c - b'A' + 27) as u32
    }
}

fn part1() -> u32 {
    INPUT
        .lines()
        .map(|line| {
            let c1 = line.bytes().take(line.len() / 2);
            let mut c2 = line.bytes().skip(line.len() / 2);
            // bitmap of chars in compartment 1
            let content1: u64 = c1.fold(0u64, |sum, c| sum | (1u64 << codechar(c)));
            // find redundant bit in compartment 2
            codechar(
                c2.find(|&c| (content1 & (1u64 << codechar(c))) > 0)
                    .unwrap(),
            )
        })
        .sum::<u32>()
}

fn part2() -> u32 {
    INPUT
        .lines()
        // need groups of 3 lines
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(|line| {
                    line.bytes()
                        // for each line set a bit for each character's code
                        .fold(0u64, |sum, c| sum | (1u64 << codechar(c)))
                })
                // bitand all 3 lines bits to find common ones
                .fold(std::u64::MAX, |acc, content| acc & content)
        })
        .map(|bits| bits.ilog2())
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
        assert_eq!(part1(), 8401);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 2641);
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
