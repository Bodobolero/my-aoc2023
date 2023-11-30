#![feature(test)]

use std::collections::HashSet;

extern crate test;
const INPUT: &str = include_str!("../inputs/input12.txt");

fn part1() -> usize {
    let mut maze: Vec<Vec<u8>> = INPUT.lines().map(|x| x.as_bytes().to_owned()).collect();
    let columns = maze[0].len();
    let rows = maze.len();
    let mut path_lengths: Vec<Vec<Option<usize>>> = vec![vec![None; columns]; rows];
    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let (mut e_x, mut e_y) = (0usize, 0usize);
    // find start and end
    for r in 0..rows {
        for c in 0..columns {
            if maze[r][c] == b'S' {
                points.insert((r, c));
                path_lengths[r][c] = Some(0);
                maze[r][c] = b'a';
            }
            if maze[r][c] == b'E' {
                (e_x, e_y) = (r, c);
                maze[r][c] = b'z';
            }
        }
    }
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut pathlen = 0;
    loop {
        // println!("pathlen: {} next points: {:?}", pathlen, points);
        if points.is_empty() {
            break;
        }
        pathlen += 1;
        let current_points = points.clone();
        points.clear();
        for p in current_points {
            for d in directions {
                let target = (p.0 as i32 + d.0, p.1 as i32 + d.1);
                if target.0 >= 0
                    && target.0 < rows as i32
                    && target.1 >= 0
                    && target.1 < columns as i32
                {
                    // we can reach this point
                    let p2 = (target.0 as usize, target.1 as usize);
                    if maze[p2.0][p2.1] <= maze[p.0][p.1] + 1 {
                        if path_lengths[p2.0][p2.1].is_none()
                            || pathlen < path_lengths[p2.0][p2.1].unwrap()
                        {
                            // we have never been here before or used a longer path so far
                            path_lengths[p2.0][p2.1] = Some(pathlen);
                            points.insert(p2);
                        }
                    }
                }
            }
        }
    }
    path_lengths[e_x][e_y].unwrap()
}

fn part2() -> usize {
    let mut maze: Vec<Vec<u8>> = INPUT.lines().map(|x| x.as_bytes().to_owned()).collect();
    let columns = maze[0].len();
    let rows = maze.len();

    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let (mut e_x, mut e_y) = (0usize, 0usize);
    let mut shortest_paths: Vec<usize> = Vec::new();
    let mut start_points: Vec<(usize, usize)> = Vec::new();
    // find start and end
    for r in 0..rows {
        for c in 0..columns {
            if maze[r][c] == b'S' || maze[r][c] == b'a' {
                maze[r][c] = b'a';
                start_points.push((r, c));
            }
            if maze[r][c] == b'E' {
                (e_x, e_y) = (r, c);
                maze[r][c] = b'z';
            }
        }
    }
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for s in start_points {
        let mut pathlen = 0;
        points.clear();
        points.insert(s);
        let mut path_lengths: Vec<Vec<Option<usize>>> = vec![vec![None; columns]; rows];
        path_lengths[s.0][s.1] = Some(0);
        loop {
            // println!("pathlen: {} next points: {:?}", pathlen, points);
            if points.is_empty() {
                break;
            }
            pathlen += 1;
            let current_points = points.clone();
            points.clear();
            for p in current_points {
                for d in directions {
                    let target = (p.0 as i32 + d.0, p.1 as i32 + d.1);
                    if target.0 >= 0
                        && target.0 < rows as i32
                        && target.1 >= 0
                        && target.1 < columns as i32
                    {
                        // we can reach this point
                        let p2 = (target.0 as usize, target.1 as usize);
                        if maze[p2.0][p2.1] <= maze[p.0][p.1] + 1 {
                            if path_lengths[p2.0][p2.1].is_none()
                                || pathlen < path_lengths[p2.0][p2.1].unwrap()
                            {
                                // we have never been here before or used a longer path so far
                                path_lengths[p2.0][p2.1] = Some(pathlen);
                                points.insert(p2);
                            }
                        }
                    }
                }
            }
        }
        if path_lengths[e_x][e_y].is_some() {
            shortest_paths.push(path_lengths[e_x][e_y].unwrap());
        }
    }
    shortest_paths.into_iter().min().unwrap()
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
        assert_eq!(part1(), 408);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 399);
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
