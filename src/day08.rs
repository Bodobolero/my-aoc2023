#![feature(test)]

extern crate test;
const INPUT: &str = include_str!("../inputs/input08.txt");

fn part1() -> usize {
    let lines: Vec<&[u8]> = INPUT.lines().map(|line| line.as_bytes()).collect();
    let (c_lines, c_columns) = (lines.len(), lines[0].len());
    let mut visible = vec![vec![false; c_columns]; c_lines];
    // make edges visible
    for i in 0..c_columns {
        visible[0][i] = true;
        visible[c_lines - 1][i] = true;
    }
    for i in 0..c_lines {
        visible[i][0] = true;
        visible[i][c_columns - 1] = true;
    }
    // println!("S {:?}", visible);

    // mark left to right
    for i in 1..c_lines - 1 {
        for j in 1..c_columns - 1 {
            let mut yes = true;
            for k in 0..j {
                if lines[i][j] <= lines[i][k] {
                    yes = false;
                    break;
                }
            }
            if yes {
                visible[i][j] = true;
            }
        }
    }
    // println!("L2R {:?}", visible);
    // mark right to left
    for i in 1..c_lines - 1 {
        for j in (1..c_columns - 1).rev() {
            let mut yes = true;
            for k in j + 1..c_columns {
                if lines[i][j] <= lines[i][k] {
                    yes = false;
                    break;
                }
            }
            if yes {
                visible[i][j] = true;
            }
        }
    }
    // println!("R2L {:?}", visible);
    // mark top to botton
    for j in 1..c_columns - 1 {
        for i in 1..c_lines - 1 {
            let mut yes = true;
            for k in 0..i {
                if lines[i][j] <= lines[k][j] {
                    yes = false;
                    break;
                }
            }
            if yes {
                visible[i][j] = true;
            }
        }
    }
    // println!("T2B {:?}", visible);

    // mark botton to top
    for j in 1..c_columns - 1 {
        for i in (1..c_lines - 1).rev() {
            let mut yes = true;
            for k in i + 1..c_lines {
                if lines[i][j] <= lines[k][j] {
                    yes = false;
                    break;
                }
            }
            if yes {
                visible[i][j] = true;
            }
        }
    }
    // println!("B2T{:?}", visible);

    // count visible trees
    visible
        .into_iter()
        .map(|line| line.into_iter().map(|b| b as usize).sum::<usize>())
        .sum::<usize>()
}

fn part2() -> usize {
    let lines: Vec<&[u8]> = INPUT.lines().map(|line| line.as_bytes()).collect();
    let (c_lines, c_columns) = (lines.len(), lines[0].len());
    let mut dist = vec![vec![1usize; c_columns]; c_lines];
    for i in 1..c_lines - 1 {
        for j in 1..c_columns - 1 {
            //left
            if j > 1 {
                let mut k = j - 1;
                while k > 0 && lines[i][k] < lines[i][j] {
                    k -= 1;
                }
                dist[i][j] *= j - k;
            }
            // println!("[{}][{}]L:{}", i, j, dist[i][j]);

            //right
            if j < c_columns - 1 {
                let mut k = j + 1;
                while k < c_columns - 1 && lines[i][k] < lines[i][j] {
                    k += 1;
                }
                dist[i][j] *= k - j;
            }
            // println!("[{}][{}]R:{}", i, j, dist[i][j]);

            // up
            if i > 1 {
                let mut k = i - 1;
                while k > 0 && lines[k][j] < lines[i][j] {
                    k -= 1;
                }
                dist[i][j] *= i - k;
            }
            // println!("[{}][{}]U:{}", i, j, dist[i][j]);

            // down
            if i < c_lines - 1 {
                let mut k = i + 1;
                while k < c_lines - 1 && lines[k][j] < lines[i][j] {
                    k += 1;
                }
                dist[i][j] *= k - i;
            }
            // println!("[{}][{}]D:{}", i, j, dist[i][j]);
        }
    }
    // println!("{:?}", dist);
    dist.into_iter()
        .map(|line| line.into_iter().max().unwrap())
        .max()
        .unwrap()
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
        assert_eq!(part1(), 1825);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 235200);
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
