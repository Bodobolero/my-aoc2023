#![feature(test)]

use itertools::Itertools;

extern crate test;

const INPUT: &str = include_str!("../inputs/input07.txt");

const ALLSPACE: usize = 70000000;
const NEEDEDSPACE: usize = 30000000;

// since rust doesn't easily allow tree preserving parent and child references
// we instead store all dirs in a vec and just the vec indexes in the parent child members of the struct
// see https://www.reddit.com/r/rust/comments/cnjhup/idiomatic_way_to_reference_parent_struct/
#[derive(Debug)]
struct Dir {
    index: usize,
    sum_offiles_size: usize, // initially flat size, in the end deep size
    sub_dirs: Vec<usize>,
    parent_dir: Option<usize>,
}

// count sum of file size in a directory
fn count_sizes(current_dir: &mut Dir, command: &str) {
    current_dir.sum_offiles_size = command
        .lines()
        .skip(1)
        .filter(|line| line.split(' ').next().unwrap() != "dir")
        .map(|line| line.split(' ').next().unwrap().parse::<usize>().unwrap())
        .sum();
}

// recursive function that adds child directory sizes to directory size
fn add_child_size(dirs: &mut Vec<Dir>, i: usize) -> usize {
    let children = dirs[i].sub_dirs.clone();
    let recursive_childsize = children
        .into_iter()
        .map(|c| add_child_size(dirs, c))
        .sum::<usize>();
    dirs[i].sum_offiles_size += recursive_childsize;
    dirs[i].sum_offiles_size
}

fn part1() -> usize {
    let mut alldirs: Vec<Dir> = Vec::new();
    let root = Dir {
        index: alldirs.len(),
        sum_offiles_size: 0,
        sub_dirs: vec![],
        parent_dir: None,
    };
    alldirs.push(root);
    let mut current_dir = 0usize;
    let commands = INPUT.split('$').skip(1);
    for command in commands {
        // println!("Parsing command: \n{}", command);
        match command.lines().next().unwrap() {
            " cd /" => current_dir = 0usize,
            " ls" => count_sizes(&mut alldirs[current_dir], command),
            " cd .." => current_dir = alldirs[current_dir].parent_dir.unwrap(),
            // cd <x>
            _ => {
                let index = alldirs.len();
                let subdir = Dir {
                    index,
                    sum_offiles_size: 0,
                    sub_dirs: Vec::new(),
                    parent_dir: Some(current_dir),
                };
                alldirs[current_dir].sub_dirs.push(index);
                current_dir = subdir.index;
                alldirs.push(subdir);
            }
        }
    }
    // println!("{:?}", alldirs);
    add_child_size(&mut alldirs, 0);
    alldirs
        .into_iter()
        .filter(|dir| dir.sum_offiles_size <= 100000)
        .map(|dir| dir.sum_offiles_size)
        .sum()
}

fn part2() -> usize {
    let mut alldirs: Vec<Dir> = Vec::new();
    let root = Dir {
        index: alldirs.len(),
        sum_offiles_size: 0,
        sub_dirs: vec![],
        parent_dir: None,
    };
    alldirs.push(root);
    let mut current_dir = 0usize;
    let commands = INPUT.split('$').skip(1);
    for command in commands {
        // println!("Parsing command: \n{}", command);
        match command.lines().next().unwrap() {
            " cd /" => current_dir = 0usize,
            " ls" => count_sizes(&mut alldirs[current_dir], command),
            " cd .." => current_dir = alldirs[current_dir].parent_dir.unwrap(),
            // cd <x>
            _ => {
                let index = alldirs.len();
                let subdir = Dir {
                    index,
                    sum_offiles_size: 0,
                    sub_dirs: Vec::new(),
                    parent_dir: Some(current_dir),
                };
                alldirs[current_dir].sub_dirs.push(index);
                current_dir = subdir.index;
                alldirs.push(subdir);
            }
        }
    }
    // println!("{:?}", alldirs);
    add_child_size(&mut alldirs, 0);
    let unused_space = ALLSPACE - alldirs[0].sum_offiles_size;
    let to_delete = NEEDEDSPACE - unused_space;
    alldirs
        .into_iter()
        .map(|dir| dir.sum_offiles_size)
        .sorted()
        .find(|dirsize| *dirsize > to_delete)
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
        assert_eq!(part1(), 1306611);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 13210366);
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
