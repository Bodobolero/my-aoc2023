#![feature(test)]

extern crate test;

const INPUT: &str = include_str!("../inputs/input05.txt");

use regex::Regex;

lazy_static::lazy_static! {
    static ref RE_MOVES: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
}

fn part1() -> String {
    let mut parts = INPUT.split("\n\n");
    let mut topology = parts.next().unwrap().lines().rev();
    let moves = parts.next().unwrap().lines();
    let stack_count = (topology.next().unwrap().len() + 2) / 4;
    println!("We have {} stacks", stack_count);
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_count];
    for line in topology {
        for i in 0..stack_count {
            let char = line.chars().nth(i * 4 + 1).unwrap();
            if char != ' ' {
                stacks[i].push(char);
            }
        }
    }
    println!("stacks: {:?}", stacks);

    for line in moves {
        let caps = RE_MOVES.captures(line).unwrap();
        let (count, from, to) = (
            caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
            caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
        );
        for _i in 0..count {
            let char = stacks[from].pop().unwrap();
            stacks[to].push(char);
        }
    }

    let mut result: String = "".to_string();
    for mut stack in stacks {
        result.push(stack.pop().unwrap());
    }
    result
}

fn part2() -> String {
    let mut parts = INPUT.split("\n\n");
    let mut topology = parts.next().unwrap().lines().rev();
    let moves = parts.next().unwrap().lines();
    let stack_count = (topology.next().unwrap().len() + 2) / 4;
    println!("We have {} stacks", stack_count);
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_count];
    for line in topology {
        for i in 0..stack_count {
            let char = line.chars().nth(i * 4 + 1).unwrap();
            if char != ' ' {
                stacks[i].push(char);
            }
        }
    }
    println!("stacks: {:?}", stacks);

    for line in moves {
        let caps = RE_MOVES.captures(line).unwrap();
        let (count, from, to) = (
            caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
            caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
        );
        let mut stack: Vec<char> = Vec::new();
        for _i in 0..count {
            let char = stacks[from].pop().unwrap();
            stack.push(char)
        }
        for _i in 0..count {
            let char = stack.pop().unwrap();
            stacks[to].push(char);
        }
    }

    let mut result: String = "".to_string();
    for mut stack in stacks {
        result.push(stack.pop().unwrap());
    }
    result
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
        assert_eq!(part1(), "HNSNMTLHQ".to_string());
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), "RNLFDJMCT".to_string());
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
