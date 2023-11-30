#![feature(test)]

use std::collections::HashMap;
use std::collections::HashSet;

extern crate test;
const INPUT: &str = include_str!("../inputs/input16.txt");

use regex::Regex;

lazy_static::lazy_static! {
    static ref RE_RULE: Regex = Regex::new(r"^Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.+)$").unwrap();
}

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    f: usize,
    target_valves: Vec<&'a str>,
}

fn parse_valves(input: &str) -> HashMap<&str, Valve> {
    let mut valves: HashMap<&str, Valve> = HashMap::new();
    for line in input.lines() {
        let caps = RE_RULE.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let flow_rate = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let targets = caps.get(3).unwrap().as_str();
        valves.insert(
            name,
            Valve {
                name,
                f: flow_rate,
                target_valves: targets.split(", ").collect::<Vec<&str>>(),
            },
        );
    }
    valves
}

fn step<'a>(
    minutes: usize,
    ppm: usize,
    pt: usize,
    current: &'a str,
    valves: &'a HashMap<&str, Valve>,
    open_valves: HashSet<&str>,
    results: &mut Vec<usize>,
    best_path: &mut HashMap<(&'a str, usize), usize>,
) {
    if minutes == 0 {
        // println!("{} {}", pt, ppm);
        results.push(pt);
    } else {
        let new_pt = pt + ppm;
        let best = best_path.get(&(current, minutes));
        if best.is_some() && *best.unwrap() > new_pt {
            return;
        }
        let entry = best_path.entry((current, minutes)).or_insert(new_pt);
        *entry = new_pt;

        let valve = valves.get(current).unwrap();
        for target in &valve.target_valves {
            step(
                minutes - 1,
                ppm,
                new_pt,
                target,
                valves,
                open_valves.clone(),
                results,
                best_path,
            );
        }
        // if valve is not yet open
        if valve.f > 0 && !open_valves.contains(current) {
            // open valve
            let mut open_valves_new = open_valves.clone();
            open_valves_new.insert(current);
            let new_ppm = ppm + valve.f;
            step(
                minutes - 1,
                new_ppm,
                new_pt,
                current,
                valves,
                open_valves_new,
                results,
                best_path,
            );
        }
    }
}

fn step2<'a>(
    minutes: usize,
    ppm: usize,
    pt: usize,
    current1: &'a str,
    current2: &'a str,
    valves: &'a HashMap<&str, Valve>,
    open_valves: HashSet<&str>,
    result: &mut usize,
    best_path: &mut HashMap<(&'a str, &'a str, usize), usize>,
) {
    if minutes == 27 {
        if pt > *result {
            *result = pt;
            // println!(
            //     "done c1 {} c2 {} ppm {} pt {} open valves {:?}",
            //     current1, current2, ppm, pt, open_valves
            // );
        }
    } else {
        let new_pt = pt + ppm;
        let best = best_path.get(&(current1, current2, minutes));
        let best2 = best_path.get(&(current2, current1, minutes));
        if (best.is_some() && *best.unwrap() >= new_pt)
            || (best2.is_some() && *best2.unwrap() >= new_pt)
        {
            return;
        }
        // println!(
        //     "min {} c1 {} c2 {} ppm {} pt {} open valves {:?}",
        //     minutes, current1, current2, ppm, pt, open_valves
        // );
        let entry = best_path
            .entry((current1, current2, minutes))
            .or_insert(new_pt);
        *entry = new_pt;

        let valve1 = valves.get(current1).unwrap();
        let valve2 = valves.get(current2).unwrap();
        if valve1.name != valve2.name
            && valve1.f > 0
            && valve2.f > 0
            && !open_valves.contains(current1)
            && !open_valves.contains(current2)
        {
            // if both valves are still closed
            // open both valves
            let mut open_valves_new = open_valves.clone();
            open_valves_new.insert(current1);
            open_valves_new.insert(current2);
            let new_ppm = ppm + valve1.f + valve2.f;
            step2(
                minutes + 1,
                new_ppm,
                new_pt,
                current1,
                current2,
                valves,
                open_valves_new,
                result,
                best_path,
            );
        }
        if valve1.f > 0 && !open_valves.contains(current1) {
            // if valve1 is still closed
            // open it
            let mut open_valves_new = open_valves.clone();
            open_valves_new.insert(current1);
            let new_ppm = ppm + valve1.f;
            for target2 in &valve2.target_valves {
                step2(
                    minutes + 1,
                    new_ppm,
                    new_pt,
                    current1,
                    target2,
                    valves,
                    open_valves_new.clone(),
                    result,
                    best_path,
                );
            }
        }
        if valve2.f > 0 && !open_valves.contains(current2) {
            // if valve2 is still closed
            // open it
            let mut open_valves_new = open_valves.clone();
            open_valves_new.insert(current2);
            let new_ppm = ppm + valve2.f;
            for target1 in &valve1.target_valves {
                step2(
                    minutes + 1,
                    new_ppm,
                    new_pt,
                    target1,
                    current2,
                    valves,
                    open_valves_new.clone(),
                    result,
                    best_path,
                );
            }
        }
        for target1 in &valve1.target_valves {
            for target2 in &valve2.target_valves {
                step2(
                    minutes + 1,
                    ppm,
                    new_pt,
                    target1,
                    target2,
                    valves,
                    open_valves.clone(),
                    result,
                    best_path,
                );
            }
        }
    }
}

fn part1() -> usize {
    let valves = parse_valves(INPUT);
    // println!("{:?}", valves);

    let pressure_per_minute: usize = 0;
    let pressure_total: usize = 0;
    let open_valves: HashSet<&str> = HashSet::new();
    let mut results: Vec<usize> = Vec::new();
    let mut best_path: HashMap<(&str, usize), usize> = HashMap::new();
    step(
        30,
        pressure_per_minute,
        pressure_total,
        "AA",
        &valves,
        open_valves.clone(),
        &mut results,
        &mut best_path,
    );

    *results.iter().max().unwrap()
}

fn part2() -> usize {
    let valves = parse_valves(INPUT);
    // println!("{:?}", valves);

    let pressure_per_minute: usize = 0;
    let pressure_total: usize = 0;
    let open_valves: HashSet<&str> = HashSet::new();
    let mut result: usize = 0;
    let mut best_path: HashMap<(&str, &str, usize), usize> = HashMap::new();
    step2(
        1,
        pressure_per_minute,
        pressure_total,
        "AA",
        "AA",
        &valves,
        open_valves.clone(),
        &mut result,
        &mut best_path,
    );

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
        assert_eq!(part1(), 1559);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 2191);
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
