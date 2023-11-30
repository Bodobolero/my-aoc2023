#![feature(test)]

extern crate test;
const INPUT: &str = include_str!("../inputs/input15.txt");

const LINE: i32 = 2000000;
const BOUND: i32 = 4000000;

use regex::Regex;
use std::collections::HashSet;

lazy_static::lazy_static! {
    static ref RE_RULE: Regex = Regex::new(r"^Sensor at x=([-+]?\d+), y=([-+]?\d+): closest beacon is at x=([-+]?\d+), y=([-+]?\d+)$").unwrap();
}

fn part1() -> usize {
    // compute vector of sensors and the manhattan distance of all points impossible
    let sensors_beacons: Vec<(i32, i32, i32, i32)> = INPUT
        .lines()
        .map(|line| {
            let caps = RE_RULE.captures(line).unwrap();
            (
                caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            )
        })
        .collect();
    let sensors_range: Vec<(i32, i32, i32)> = sensors_beacons
        .iter()
        .map(|(sx, sy, bx, by)| {
            // compute manhattan distance of beacon
            (*sx, *sy, (bx - sx).abs() + (by - sy).abs())
        })
        .collect();
    let beacons: HashSet<(i32, i32)> = sensors_beacons
        .iter()
        .map(|(_, _, bx, by)| (*bx, *by))
        .collect();
    // the line at y=LINE potentially is infite - we need to determine
    // where we start to compute the positions covered
    let sx_min = *sensors_range.iter().map(|(x, _, _)| x).min().unwrap();
    let sx_max = *sensors_range.iter().map(|(x, _, _)| x).max().unwrap();
    let max_dist = *sensors_range.iter().map(|(_, _, d)| d).max().unwrap();
    let mut count: usize = 0;
    for x in sx_min - max_dist..sx_max + max_dist {
        if !beacons.contains(&(x, LINE)) {
            // compute manhattan distance of position from all sensors and check if it falls in range
            for (sx, sy, range) in &sensors_range {
                let dist = (x - *sx).abs() + (LINE - *sy).abs();
                if dist <= *range {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

fn part2() -> usize {
    // compute vector of sensors and the manhattan distance of all points impossible
    let sensors_beacons: Vec<(i32, i32, i32, i32)> = INPUT
        .lines()
        .map(|line| {
            let caps = RE_RULE.captures(line).unwrap();
            (
                caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            )
        })
        .collect();
    let sensors_range: Vec<(i32, i32, i32)> = sensors_beacons
        .iter()
        .map(|(sx, sy, bx, by)| {
            // compute manhattan distance of beacon
            (*sx, *sy, (bx - sx).abs() + (by - sy).abs())
        })
        .collect();

    // for each sensor find the first point that overlaps its range and the last point
    // skip all points in betwen to be faster
    let (mut sol_x, mut sol_y) = (0i32, 0i32);

    'outer: for y in 0..BOUND + 1 {
        let mut ranges: Vec<(i32, i32)> = Vec::new();
        for (sx, sy, range) in &sensors_range {
            if (y - *sy).abs() > *range {
                // sensor area does not touch line
                continue;
            }
            // sensor touches the following line range:
            let xdelta = range - (y - *sy).abs();
            let xmin = *sx - xdelta;
            let xmax = *sx + xdelta;
            ranges.push((xmin, xmax));
        }
        ranges.sort();
        // println!("line {} has ranges {:?}", y, ranges);
        let mut pos = 0;
        for (x1, x2) in ranges {
            if pos < x1 {
                // found it
                sol_x = pos;
                sol_y = y;
                break 'outer;
            }
            if pos <= x2 {
                pos = x2 + 1;
            }
        }
        if pos <= BOUND {
            // found it
            sol_x = pos;
            sol_y = y;
            break 'outer;
        }
    }
    // println!("found x:{} y:{}", sol_x, sol_y);

    4000000usize * (sol_x as usize) + (sol_y as usize)
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
        assert_eq!(part1(), 4951427);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 13029714573243);
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
