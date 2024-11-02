#![feature(test)]

extern crate test;

const INPUT: &str = include_str!("../inputs/input06.txt");

fn number_wins(time: u32, distance: u32) -> usize {
    (0..time).filter(|t| t * (time - t) > distance).count()
}

pub fn part1() -> usize {
    let mut lines = INPUT.lines();
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .filter_map(|s| s.parse::<u32>().ok()) // Parses each item and skips non-integer values
        .collect::<Vec<u32>>();
    let distances = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .filter_map(|s| s.parse::<u32>().ok()) // Parses each item and skips non-integer values
        .collect::<Vec<u32>>();
    assert!(times.len() == distances.len());
    let races = times.into_iter().zip(distances);

    races
        .map(|(time, distance)| number_wins(time, distance))
        .product()
}

fn binary_find_first(max: u64, distance: u64) -> Option<u64> {
    let mut low = 0;
    let mut high = max;

    while low < high {
        let mid = (low + high) / 2;

        if mid * (max - mid) > distance {
            high = mid; // Narrow down to the lower half, as we may have more matches earlier
        } else {
            low = mid + 1; // Narrow down to the upper half
        }
    }

    // After the loop, `low` is the first position where `condition` is true, if any
    if low < max && low * (max - low) > distance {
        Some(low)
    } else {
        None
    }
}

fn binary_find_last(max: u64, distance: u64) -> Option<u64> {
    let mut low = 0;
    let mut high = max;

    while low < high {
        let mid = (low + high + 1) / 2; // Adjust to round up

        if mid * (max - mid) > distance {
            low = mid; // Narrow down to the upper half, as we may have more matches later
        } else {
            high = mid - 1; // Narrow down to the lower half
        }
    }

    // After the loop, `low` is the last position where `condition` is true, if any
    if low < max && low * (max - low) > distance {
        Some(low)
    } else {
        None
    }
}

pub fn part2() -> u64 {
    let time = INPUT
        .lines()
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = INPUT
        .lines()
        .nth(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    // let first_time = (0..time)
    //     .find(|t| t * (time - t) > distance)
    //     .unwrap_or_default();
    // let last_time = (1..=time)
    //     .rev()
    //     .find(|t| t * (time - t) > distance)
    //     .unwrap_or_default();
    let first_time = binary_find_first(time, distance).unwrap();
    let last_time = binary_find_last(time, distance).unwrap();
    assert!(last_time >= first_time);
    last_time - first_time + 1
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
        assert_eq!(part1(), 252000);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 36992486);
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
