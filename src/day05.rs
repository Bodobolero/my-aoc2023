#![feature(test)]

use rustc_hash::FxHashMap;
use std::collections::HashMap;

extern crate test;

const INPUT: &str = include_str!("../inputs/input05.txt");

#[derive(Debug)]
pub struct Map<'a> {
    to: &'a str,
    // a range is a tuple (destination range start, source range start, range length)
    ranges: Vec<(usize, usize, usize)>,
    history: FxHashMap<usize, usize>,
}

// a map with "from" as key and the map as value
type MapMap<'a> = HashMap<&'a str, Map<'a>>;

// map seed to location
fn map(seed: usize, mapmap: &mut MapMap) -> usize {
    let mut from = "seed";
    let mut value = seed;

    'outer: loop {
        let map = mapmap.get_mut(from).unwrap();
        if let Some(mapped) = map.history.get(&value) {
            print!("+");
            value = *mapped;
            break 'outer;
        }
        let mut new_value = value;
        'inner: for (dest_range_start, src_range_start, range_len) in &map.ranges {
            if value >= *src_range_start && value < *src_range_start + range_len {
                new_value = *dest_range_start + (value - *src_range_start);
                break 'inner;
            }
        }
        map.history.insert(value, new_value);
        print!("-");
        value = new_value;
        if map.to == "location" {
            break 'outer;
        }
        from = map.to;
    }
    value
}

fn parse_input<'a>(input: &'a str) -> (Vec<usize>, MapMap) {
    let mut mapmap: MapMap<'a> = HashMap::new();
    let mut sections = input.split("\n\n");
    // parse seeds
    let seeds_str = sections.next().unwrap();
    let mut seeds_split = seeds_str.split(": ");
    seeds_split.next();
    let seeds: Vec<usize> = seeds_split
        .next()
        .unwrap()
        .split(' ')
        .map(|nr_as_str| nr_as_str.parse::<usize>().unwrap())
        .collect();
    for section in sections {
        let mut lines = section.lines();
        let mut name_split = lines.next().unwrap().split(' ');
        name_split = name_split.next().unwrap().split('-');
        let from = name_split.next().unwrap();
        name_split.next();
        let to = name_split.next().unwrap();
        // println!("map from {} to {}", from, to);
        let ranges: Vec<(usize, usize, usize)> = lines
            .map(|s| {
                let mut nrs = s.split(' ').map(|nr_str| nr_str.parse::<usize>().unwrap());
                let dest_range_start = nrs.next().unwrap();
                let src_range_start = nrs.next().unwrap();
                let range_len = nrs.next().unwrap();
                (dest_range_start, src_range_start, range_len)
            })
            .collect();
        let map = Map {
            to,
            ranges,
            history: FxHashMap::default(),
        };
        // println!("{:?}", map);
        mapmap.insert(from, map);
    }

    (seeds, mapmap)
}

fn part1(input: &str) -> usize {
    let (seeds, mut mapmap) = parse_input(input);
    // println!("{:?}", seeds);
    seeds
        .into_iter()
        .map(|seed| map(seed, &mut mapmap))
        .min()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let (seeds, mut mapmap) = parse_input(input);
    let mut local_minima: Vec<usize> = Vec::new();
    for i in 0..seeds.len() / 2 {
        let local_minimum = (seeds[i * 2]..seeds[i * 2] + seeds[i * 2 + 1])
            .map(|seed| map(seed, &mut mapmap))
            .min()
            .unwrap();
        println!(
            "Local minimum: {} for range {}..{}",
            local_minimum,
            seeds[i * 2],
            seeds[i * 2] + seeds[i * 2 + 1]
        );
        local_minima.push(local_minimum);
    }
    *local_minima.iter().min().unwrap()
}

pub fn main() {
    println!("Part 1: Answer {}", part1(INPUT));
    println!("Part 2: Answer {} ", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const EXAMPLE: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 35);
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(INPUT), 174137457);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 46);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(INPUT), 0);
    }
    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(|| part1(INPUT));
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        b.iter(|| part2(INPUT));
    }
}
