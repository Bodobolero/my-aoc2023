#![feature(test)]

extern crate test;
const INPUT: &str = include_str!("../inputs/input14.txt");

use regex::Regex;

lazy_static::lazy_static! {
    static ref RE_POINT: Regex = Regex::new(r"(\d+),(\d+)").unwrap();
}

#[derive(Debug, Clone, PartialEq, Copy)]
#[repr(u8)]
enum Mode {
    Air = b'.',
    Rock = b'#',
    Sand = b'o',
}

fn print_grid(grid: &Vec<Vec<Mode>>) {
    for line in grid {
        for pos in line {
            print!("{}", *pos as u8 as char);
        }
        println!();
    }
}

fn part1() -> usize {
    let linepoints: Vec<Vec<(usize, usize)>> = INPUT
        .lines()
        .map(|line| {
            let mut points: Vec<(usize, usize)> = Vec::new();
            for pointcap in RE_POINT.captures_iter(line) {
                points.push((
                    pointcap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    pointcap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                ));
            }
            // println!("points: {:?}", points);
            points
        })
        .collect();
    let xmin = linepoints
        .iter()
        .flat_map(|v| v.iter())
        .map(|p| p.0)
        .min()
        .unwrap();
    let xmax = linepoints
        .iter()
        .flat_map(|v| v.iter())
        .map(|p| p.0)
        .max()
        .unwrap();
    let ymax = linepoints
        .iter()
        .flat_map(|v| v.iter())
        .map(|p| p.1)
        .max()
        .unwrap();
    // println!("x: {}..={} y: 0..{}", xmin, xmax, ymax);
    let mut grid: Vec<Vec<Mode>> = vec![vec![Mode::Air; xmax - xmin + 3]; ymax + 2];
    // draw rocks
    for points in linepoints {
        for i in 1..points.len() {
            let mut pos = points[i - 1];
            let mut end = points[i];
            if pos.0 > end.0 || pos.1 > end.1 {
                std::mem::swap(&mut end, &mut pos);
            }
            // println!("Grid before drawing line from {:?} to {:?}", pos, end);
            // printGrid(&grid);
            let step = (usize::from(end.0 > pos.0), usize::from(end.1 > pos.1));
            while pos.0 <= end.0 && pos.1 <= end.1 {
                grid[pos.1][pos.0 - xmin + 1] = Mode::Rock;
                pos = (pos.0 + step.0, pos.1 + step.1);
            }
        }
    }
    // println!("Grid after drawing all lines");
    // printGrid(&grid);
    // collect sand
    let mut sand: usize = 0;
    'outer: loop {
        let mut pos = (500 - xmin + 1, 0);
        while pos.1 <= ymax {
            if grid[pos.1 + 1][pos.0] == Mode::Air {
                pos = (pos.0, pos.1 + 1);
                continue;
            }
            if pos.0 > 0 && grid[pos.1 + 1][pos.0 - 1] == Mode::Air {
                pos = (pos.0 - 1, pos.1 + 1);
                continue;
            }
            if pos.0 < xmax + 2 && grid[pos.1 + 1][pos.0 + 1] == Mode::Air {
                pos = (pos.0 + 1, pos.1 + 1);
                continue;
            }
            grid[pos.1][pos.0] = Mode::Sand;
            sand += 1;

            continue 'outer;
        }
        break 'outer;
    }
    println!("Grid with {sand} corns of sand:");
    print_grid(&grid);
    sand
}

fn part2() -> usize {
    let linepoints: Vec<Vec<(usize, usize)>> = INPUT
        .lines()
        .map(|line| {
            let mut points: Vec<(usize, usize)> = Vec::new();
            for pointcap in RE_POINT.captures_iter(line) {
                points.push((
                    pointcap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    pointcap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                ));
            }
            // println!("points: {:?}", points);
            points
        })
        .collect();

    let ymax = linepoints
        .iter()
        .flat_map(|v| v.iter())
        .map(|p| p.1)
        .max()
        .unwrap()
        + 3;
    let xmin = 500 - ymax - 1;
    let xmax = 500 + ymax + 1;

    // println!("x: {}..={} y: 0..{}", xmin, xmax, ymax);
    let mut grid: Vec<Vec<Mode>> = vec![vec![Mode::Air; xmax - xmin]; ymax];
    // draw rocks
    for points in linepoints {
        for i in 1..points.len() {
            let mut pos = points[i - 1];
            let mut end = points[i];
            if pos.0 > end.0 || pos.1 > end.1 {
                std::mem::swap(&mut end, &mut pos);
            }
            // println!("Grid before drawing line from {:?} to {:?}", pos, end);
            // printGrid(&grid);
            let step = (usize::from(end.0 > pos.0), usize::from(end.1 > pos.1));
            while pos.0 <= end.0 && pos.1 <= end.1 {
                grid[pos.1][pos.0 - xmin] = Mode::Rock;
                pos = (pos.0 + step.0, pos.1 + step.1);
            }
        }
    }
    // draw bottom line
    for i in 0..grid[ymax - 1].len() {
        grid[ymax - 1][i] = Mode::Rock;
    }
    // println!("Grid after drawing all lines");
    // printGrid(&grid);
    // collect sand
    let mut sand: usize = 0;
    'outer: loop {
        let mut pos = (500 - xmin + 1, 0);
        if grid[pos.1][pos.0] == Mode::Sand {
            break;
        }
        while pos.1 < ymax {
            if grid[pos.1 + 1][pos.0] == Mode::Air {
                pos = (pos.0, pos.1 + 1);
                continue;
            }
            if pos.0 > 0 && grid[pos.1 + 1][pos.0 - 1] == Mode::Air {
                pos = (pos.0 - 1, pos.1 + 1);
                continue;
            }
            if pos.0 < xmax + 2 && grid[pos.1 + 1][pos.0 + 1] == Mode::Air {
                pos = (pos.0 + 1, pos.1 + 1);
                continue;
            }
            grid[pos.1][pos.0] = Mode::Sand;
            sand += 1;
            // println!("Grid with {} corns of sand:", sand);
            // printGrid(&grid);
            continue 'outer;
        }
    }
    println!("Grid with {sand} corns of sand:");
    print_grid(&grid);
    sand
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
        assert_eq!(part1(), 964);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 32041);
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
