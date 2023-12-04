#![feature(test)]

use rustc_hash::{FxHashMap, FxHashSet};

extern crate test;

const INPUT: &str = include_str!("../inputs/input03.txt");

fn is_symbol(tocheck: u8) -> bool {
    tocheck != b'.' && !tocheck.is_ascii_digit()
}
fn symbol_adjacent(bytes: &Vec<&[u8]>, line: usize, startpos: usize, endpos: usize) -> bool {
    // check left
    if startpos > 0 && is_symbol(bytes[line][startpos - 1]) {
        return true;
    }
    // check right
    if endpos + 1 < bytes[line].len() && is_symbol(bytes[line][endpos + 1]) {
        return true;
    }
    let start = if startpos == 0 { 1 } else { startpos };
    let end = if endpos + 1 == bytes[line].len() {
        endpos - 1
    } else {
        endpos
    };
    // check top
    if line > 0 {
        for i in start - 1..end + 2 {
            if is_symbol(bytes[line - 1][i]) {
                return true;
            }
        }
    }
    // check bottom
    if line + 1 < bytes.len() {
        for i in start - 1..end + 2 {
            if is_symbol(bytes[line + 1][i]) {
                return true;
            }
        }
    }
    false
}

fn part1() -> u32 {
    let mut sum: u32 = 0;
    let bytes: Vec<&[u8]> = INPUT.lines().map(|l| l.as_bytes()).collect();
    for line in 0..bytes.len() {
        let mut pos: usize = 0;
        let mut number: u32;
        while pos < bytes[line].len() {
            if bytes[line][pos].is_ascii_digit() {
                let start_nr_pos = pos;
                number = (bytes[line][pos] - b'0') as u32;
                pos += 1;
                while pos < bytes[line].len() && bytes[line][pos].is_ascii_digit() {
                    number = number * 10 + (bytes[line][pos] - b'0') as u32;
                    pos += 1;
                }
                // print!("parsed number {}", number);
                if symbol_adjacent(&bytes, line, start_nr_pos, pos - 1) {
                    sum += number;
                    // println!(" is adjacent to symbol");
                }
                //  else {
                //     println!(" is ignored and NOT adjacent to symbol");
                // }
            }
            pos += 1;
        }
    }
    sum
}

fn is_star(tocheck: u8) -> bool {
    tocheck == b'*'
}

fn find_adjacent_stars(
    bytes: &Vec<&[u8]>,
    line: usize,
    startpos: usize,
    endpos: usize,
) -> Vec<(usize, usize)> {
    let mut stars: Vec<(usize, usize)> = Vec::new();

    // check left
    if startpos > 0 && is_star(bytes[line][startpos - 1]) {
        stars.push((line, startpos - 1));
    }
    // check right
    if endpos + 1 < bytes[line].len() && is_star(bytes[line][endpos + 1]) {
        stars.push((line, endpos + 1));
    }
    let start = if startpos == 0 { 1 } else { startpos };
    let end = if endpos + 1 == bytes[line].len() {
        endpos - 1
    } else {
        endpos
    };
    // check top
    if line > 0 {
        for i in start - 1..end + 2 {
            if is_symbol(bytes[line - 1][i]) {
                stars.push((line - 1, i));
            }
        }
    }
    // check bottom
    if line + 1 < bytes.len() {
        for i in start - 1..end + 2 {
            if is_symbol(bytes[line + 1][i]) {
                stars.push((line + 1, i));
            }
        }
    }
    stars
}

type StarPos = (usize, usize);
type Number = (u32, usize, usize);
type NumberSet = FxHashSet<Number>;
type StarMap = FxHashMap<StarPos, NumberSet>;

fn part2() -> u32 {
    let mut star_map: StarMap = FxHashMap::default();
    let bytes: Vec<&[u8]> = INPUT.lines().map(|l| l.as_bytes()).collect();
    for line in 0..bytes.len() {
        let mut pos: usize = 0;
        let mut number: u32;
        while pos < bytes[line].len() {
            if bytes[line][pos].is_ascii_digit() {
                let start_nr_pos = pos;
                number = (bytes[line][pos] - b'0') as u32;
                pos += 1;
                while pos < bytes[line].len() && bytes[line][pos].is_ascii_digit() {
                    number = number * 10 + (bytes[line][pos] - b'0') as u32;
                    pos += 1;
                }
                let stars = find_adjacent_stars(&bytes, line, start_nr_pos, pos - 1);
                for star in stars {
                    let set = star_map.entry(star).or_default();
                    set.insert((number, line, start_nr_pos));
                }
            }
            pos += 1;
        }
    }
    // println!("Stars {:?}", star_map);
    star_map
        .values()
        .filter(|set| set.len() > 1)
        .map(|set| set.iter().fold(1, |acc, &x| acc * x.0))
        .sum::<u32>()
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
        assert_eq!(part1(), 556367);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 89471771);
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
