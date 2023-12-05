#![feature(test)]

use std::{cmp::Ordering, collections::HashMap};

extern crate test;

const INPUT: &str = include_str!("../inputs/input05.txt");

#[derive(Debug)]
pub struct Map<'a> {
    to: &'a str,
    // a range is a tuple (destination range start, source range start, range length)
    ranges: Vec<(usize, usize, usize)>,
}

// a map with "from" as key and the map as value
type MapMap<'a> = HashMap<&'a str, Map<'a>>;

// map seed to location
fn map(seed: usize, mapmap: &MapMap) -> usize {
    let mut from = "seed";
    let mut value = seed;

    'outer: loop {
        let map = mapmap.get(from).unwrap();
        // now that we sorted the ranges we can do a quicksearch
        if let Ok(pos) = map
            .ranges
            .binary_search_by(|&(_, src_range_start, range_len)| {
                if value < src_range_start {
                    return Ordering::Greater;
                };
                if value < src_range_start + range_len {
                    return Ordering::Equal;
                };
                Ordering::Less
            })
        {
            let (dest_range_start, src_range_start, _) = map.ranges[pos];
            value = dest_range_start + (value - src_range_start);
        }
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
        let mut map = Map { to, ranges };
        map.ranges
            .sort_by(|(_, src_start_a, _), (_, src_start_b, _)| src_start_a.cmp(src_start_b));
        // println!("{:?}", map);
        mapmap.insert(from, map);
    }

    (seeds, mapmap)
}

// return ranges
fn map_ranges(ranges: &[usize], mapmap: &MapMap) -> Vec<usize> {
    let mut from = "seed";
    let mut ranges: Vec<usize> = ranges.to_vec();
    let mut mapped_ranges: Vec<usize> = Vec::new();

    'outer: loop {
        let map = mapmap.get(from).unwrap();
        // println!("processing map {:?}", map);
        // println!("source ranges: {:?}", ranges);
        'inner: for i in 0..ranges.len() / 2 {
            let mut start = ranges[i * 2];
            let mut len = ranges[i * 2 + 1];
            loop {
                // search start of range
                match map
                    .ranges
                    .binary_search_by(|&(_, src_range_start, range_len)| {
                        if start < src_range_start {
                            return Ordering::Greater;
                        };
                        if start < src_range_start + range_len {
                            return Ordering::Equal;
                        };
                        Ordering::Less
                    }) {
                    Ok(pos) => {
                        let (dest_range_start, src_range_start, range_len) = map.ranges[pos];
                        let target_src_range_start = dest_range_start + (start - src_range_start);
                        let rest_len = range_len - (start - src_range_start);
                        if len <= rest_len {
                            // we have a simple mapped range and have fully mapped this range
                            mapped_ranges.push(target_src_range_start);
                            mapped_ranges.push(len);
                            continue 'inner;
                        } else {
                            mapped_ranges.push(target_src_range_start);
                            mapped_ranges.push(rest_len);
                            start += rest_len;
                            len -= rest_len;
                        }
                    }
                    Err(pos) => {
                        if pos >= map.ranges.len() {
                            // we are at the end of all ranges
                            mapped_ranges.push(start);
                            mapped_ranges.push(len);
                            continue 'inner;
                        } else {
                            // we are smaller then the next found range
                            let (_, src_range_start, _) = map.ranges[pos];
                            assert!(start < src_range_start);
                            // add identical range to output
                            mapped_ranges.push(start);
                            if start + len <= src_range_start {
                                mapped_ranges.push(len);
                                continue 'inner; // complete range mapped
                            } else {
                                let mapped_len = src_range_start - start; // only interval until next mapped range
                                mapped_ranges.push(mapped_len);
                                start += mapped_len;
                                len -= mapped_len;
                                // iterate loop with rest of range to be mapped
                            }
                        }
                    }
                }
            }
        }
        ranges = mapped_ranges;
        // println!("mapped ranges {:?}", ranges);
        mapped_ranges = Vec::new();

        if map.to == "location" {
            break 'outer;
        }
        from = map.to;
    }
    ranges
}

fn part1(input: &str) -> usize {
    let (seeds, mapmap) = parse_input(input);
    // println!("{:?}", seeds);
    seeds
        .into_iter()
        .map(|seed| map(seed, &mapmap))
        .min()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let (ranges, mapmap) = parse_input(input);
    let output_ranges = map_ranges(&ranges, &mapmap);
    *output_ranges.iter().step_by(2).min().unwrap()
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
        assert_eq!(part2(INPUT), 1493866);
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
