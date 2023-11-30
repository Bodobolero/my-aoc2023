#![feature(test)]

extern crate test;
const INPUT: &str = include_str!("../inputs/input09.txt");

use std::collections::HashSet;

fn follow(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    // If the head is ever two steps directly up, down, left, or right from the tail,
    //the tail must also move one step in that direction so it remains close enough:
    if head.0 == tail.0 && (head.1 - tail.1).abs() == 2 {
        return (tail.0, tail.1 + (head.1 - tail.1) / 2);
    }
    if head.1 == tail.1 && (head.0 - tail.0).abs() == 2 {
        return (tail.0 + (head.0 - tail.0) / 2, tail.1);
    }

    //Otherwise, if the head and tail aren't touching and aren't in the same row or column,
    //the tail always moves one step diagonally to keep up:
    if (head.0 - tail.0).abs() == 1 && (head.1 - tail.1).abs() == 2 {
        return (head.0, tail.1 + (head.1 - tail.1) / 2);
    }
    if (head.1 - tail.1).abs() == 1 && (head.0 - tail.0).abs() == 2 {
        return (tail.0 + (head.0 - tail.0) / 2, head.1);
    }
    // new moves in part 2
    // here we have a distance of 2 in x and y
    if (head.0 - tail.0).abs() == 2 && (head.1 - tail.1).abs() == 2 {
        return (
            tail.0 + (head.0 - tail.0) / 2,
            tail.1 + (head.1 - tail.1) / 2,
        );
    }
    if (head.1 - tail.1).abs() == 2 && (head.0 - tail.0).abs() == 2 {
        return (
            tail.0 + (head.0 - tail.0) / 2,
            tail.1 + (head.1 - tail.1) / 2,
        );
    }

    // otherwise do nothing
    tail
}

fn part1() -> usize {
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    let mut tail = (0, 0);
    let mut head = (0, 0);
    tail_visited.insert((tail.0, tail.1));

    for line in INPUT.lines() {
        let mut parse = line.split(" ");
        let direction = parse.next().unwrap();
        let amount = parse.next().unwrap().parse::<i32>().unwrap();
        let (x, y) = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => panic!("unexpected direction"),
        };
        for _ in 0..amount {
            head = (head.0 + x, head.1 + y);
            tail = follow(head, tail);
            tail_visited.insert((tail.0, tail.1));
        }
    }

    tail_visited.len()
}

fn part2() -> usize {
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    let mut knots = vec![(0, 0); 10];
    tail_visited.insert((0, 0));

    for line in INPUT.lines() {
        let mut parse = line.split(" ");
        let direction = parse.next().unwrap();
        let amount = parse.next().unwrap().parse::<i32>().unwrap();
        let (x, y) = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => panic!("unexpected direction"),
        };
        for _ in 0..amount {
            knots[0] = (knots[0].0 + x, knots[0].1 + y);
            for knot in 1..10 {
                knots[knot] = follow(knots[knot - 1], knots[knot]);
            }
            tail_visited.insert(knots[9]);
        }
    }

    tail_visited.len()
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
        assert_eq!(part1(), 6026);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 2273);
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
