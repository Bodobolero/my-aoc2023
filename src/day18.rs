#![feature(test)]

extern crate test;
const INPUT: &str = include_str!("../inputs/input18.txt");

fn scan_cubes() -> Vec<[i8; 3]> {
    INPUT
        .lines()
        .map(|line| {
            let mut num_iterator = line.split(',').map(|s| s.parse::<i8>().unwrap());
            [
                num_iterator.next().unwrap() + 1,
                num_iterator.next().unwrap() + 1,
                num_iterator.next().unwrap() + 1,
            ]
        })
        .collect()
}

fn part1() -> usize {
    let cubes = scan_cubes();
    let mut free_sides: usize = 0;
    for c1 in &cubes {
        let mut c1_free = 6;
        for c2 in &cubes {
            let distance = (c1[0] - c2[0]).abs() + (c1[1] - c2[1]).abs() + (c1[2] - c2[2]).abs();
            if distance == 1 {
                c1_free -= 1;
            }
        }
        free_sides += c1_free;
    }
    free_sides
}

#[derive(Clone, PartialEq)]
enum Droplet {
    Air,
    Lava,
    Steam,
}

fn steam(x: usize, y: usize, z: usize, grid: &mut Vec<Vec<Vec<Droplet>>>, minmax: &[usize; 6]) {
    let directions: [[i32; 3]; 6] = [
        [-1, 0, 0],
        [1, 0, 0],
        [0, -1, 0],
        [0, 1, 0],
        [0, 0, -1],
        [0, 0, 1],
    ];
    'dir: for d in directions {
        let newpos = [x as i32 + d[0], y as i32 + d[1], z as i32 + d[2]];
        for i in 0..3 {
            if newpos[i] < minmax[i * 2] as i32 {
                continue 'dir;
            }
            if newpos[i] > minmax[i * 2 + 1] as i32 {
                continue 'dir;
            }
        }
        // println!("check {:?} for lava", newpos);
        if grid[newpos[0] as usize][newpos[1] as usize][newpos[2] as usize] == Droplet::Air {
            // println!("steam {:?}", newpos);
            grid[newpos[0] as usize][newpos[1] as usize][newpos[2] as usize] = Droplet::Steam;
            steam(
                newpos[0] as usize,
                newpos[1] as usize,
                newpos[2] as usize,
                grid,
                minmax,
            );
        }
    }
}

fn part2() -> usize {
    let cubes = scan_cubes();
    // minimum for sides: -x, +x, -y, +y, -z, +z
    let minmax = [
        (cubes.iter().map(|c| c[0]).min().unwrap() - 1) as usize,
        (cubes.iter().map(|c| c[0]).max().unwrap() + 1) as usize,
        (cubes.iter().map(|c| c[1]).min().unwrap() - 1) as usize,
        (cubes.iter().map(|c| c[1]).max().unwrap() + 1) as usize,
        (cubes.iter().map(|c| c[2]).min().unwrap() - 1) as usize,
        (cubes.iter().map(|c| c[2]).max().unwrap() + 1) as usize,
    ];
    // println!("minmax: {:?}", minmax);

    // we surround by one layer of steam
    let mut grid: Vec<Vec<Vec<Droplet>>> =
        vec![
            vec![vec![Droplet::Air; minmax[5] - minmax[4] + 2]; minmax[3] - minmax[2] + 2];
            minmax[1] - minmax[0] + 2
        ];
    for c in &cubes {
        grid[c[0] as usize][c[1] as usize][c[2] as usize] = Droplet::Lava;
    }
    grid[minmax[0]][minmax[2]][minmax[4]] = Droplet::Steam;
    steam(minmax[0], minmax[2], minmax[4], &mut grid, &minmax);
    let mut steam_sides: usize = 0;
    let directions: [[i32; 3]; 6] = [
        [-1, 0, 0],
        [1, 0, 0],
        [0, -1, 0],
        [0, 1, 0],
        [0, 0, -1],
        [0, 0, 1],
    ];

    for c in &cubes {
        for d in directions {
            let newpos = [c[0] as i32 + d[0], c[1] as i32 + d[1], c[2] as i32 + d[2]];

            if grid[newpos[0] as usize][newpos[1] as usize][newpos[2] as usize] == Droplet::Steam {
                steam_sides += 1;
            }
        }
    }

    steam_sides
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
        assert_eq!(part1(), 3564);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 2106);
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
