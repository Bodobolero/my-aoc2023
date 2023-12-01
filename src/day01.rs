#![feature(test)]

use lazy_static::lazy_static;
use std::collections::HashMap;

extern crate test;
const INPUT: &str = include_str!("../inputs/input01.txt");

fn find_first_digit(line: &str) -> Option<char> {
    line.chars().find(|c| c.is_numeric())
}
fn find_last_digit(line: &str) -> Option<char> {
    line.chars().rev().find(|c| c.is_numeric())
}

fn find_number(line: &str) -> u32 {
    let first_digit = find_first_digit(line).unwrap() as u8 - b'0';
    let last_digit = find_last_digit(line).unwrap() as u8 - b'0';
    first_digit as u32 * 10 + last_digit as u32
}

fn part1() -> u32 {
    INPUT.lines().map(find_number).sum::<u32>()
}

fn find_first_digit_and_its_position(line: &str) -> Option<(usize, u8)> {
    line.chars()
        .enumerate()
        .find(|c| c.1.is_numeric())
        .map(|(pos, c)| (pos, c as u8 - b'0'))
}

fn find_last_digit_and_its_position(line: &str) -> Option<(usize, u8)> {
    line.chars()
        .rev()
        .enumerate()
        .find(|c| c.1.is_numeric())
        .map(|(pos, c)| (line.len() - pos, c as u8 - b'0'))
}

static DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_real_number(line: &str) -> u32 {
    let mut d1: u32 = 0;
    'outer: for i in 0..line.len() {
        if (line.as_bytes()[i] as char).is_numeric() {
            d1 = (line.as_bytes()[i] - b'0') as u32;
            break 'outer;
        }
        for (pos, d) in DIGITS.into_iter().enumerate() {
            if i + d.len() > line.len() {
                continue;
            }
            if *d == line[i..i + d.len()] {
                d1 = pos as u32 + 1;
                break 'outer;
            }
        }
    }
    let mut d2: u32 = 0;
    'outer: for i in (0..line.len()).rev() {
        if (line.as_bytes()[i] as char).is_numeric() {
            d2 = (line.as_bytes()[i] - b'0') as u32;
            break 'outer;
        }
        for (pos, d) in DIGITS.into_iter().enumerate() {
            if i + d.len() > line.len() {
                continue;
            }
            if *d == line[i..i + d.len()] {
                d2 = pos as u32 + 1;
                break 'outer;
            }
        }
    }
    d1 * 10 + d2
}

fn part2() -> u32 {
    INPUT.lines().map(find_real_number).sum::<u32>()
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
        assert_eq!(part1(), 54877);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 54100);
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
