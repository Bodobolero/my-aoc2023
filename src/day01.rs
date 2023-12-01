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

lazy_static! {
    static ref DIGITS: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("one", 1);
        m.insert("two", 2);
        m.insert("three", 3);
        m.insert("four", 4);
        m.insert("five", 5);
        m.insert("six", 6);
        m.insert("seven", 7);
        m.insert("eight", 8);
        m.insert("nine", 9);
        m
    };
}

fn find_first_word_and_its_position(line: &str) -> Option<(usize, u8)> {
    DIGITS
        .iter()
        .map(|(k, v)| (line.find(k), v))
        .filter(|(o, v)| o.is_some())
        .map(|(o, v)| (o.unwrap(), *v))
        .min()
}

fn find_last_word_and_its_position(line: &str) -> Option<(usize, u8)> {
    DIGITS
        .iter()
        .map(|(k, v)| (line.rfind(k), v))
        .filter(|(o, v)| o.is_some())
        .map(|(o, v)| (o.unwrap(), *v))
        .max()
}

fn find_real_number(line: &str) -> u32 {
    let first_digit = find_first_digit_and_its_position(line);
    let first_word = find_first_word_and_its_position(line);
    let last_digit = find_last_digit_and_its_position(line);
    let last_word = find_last_word_and_its_position(line);
    // println!("line: {}", line);
    // println!(
    //     "first_digit {:?} first_word {:?} last_digit {:?} last_word {:?}",
    //     first_digit, first_word, last_digit, last_word
    // );
    let mut d1: u32 = 0;
    let mut d2: u32 = 0;
    let d1 = match (first_digit, first_word) {
        (Some(t1), Some(t2)) => {
            if t1.0 < t2.0 {
                t1.1
            } else {
                t2.1
            }
        }
        (Some(t1), None) => t1.1,
        (None, Some(t2)) => t2.1,
        _ => panic!("no first digit found in line {}", line),
    };
    let d2 = match (last_digit, last_word) {
        (Some(t1), Some(t2)) => {
            if t1.0 > t2.0 {
                t1.1
            } else {
                t2.1
            }
        }
        (Some(t1), None) => t1.1,
        (None, Some(t2)) => t2.1,
        _ => panic!("no last digit found in line {}", line),
    };
    // println!("{}", d1 as u32 * 10 + d2 as u32);
    d1 as u32 * 10 + d2 as u32
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
