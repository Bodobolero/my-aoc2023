#![feature(test)]

use std::collections::HashMap;

extern crate test;

const INPUT: &str = include_str!("../inputs/input02.txt");

fn part1() -> u32 {
    let mut allowed = HashMap::new();
    allowed.insert("red", 12);
    allowed.insert("green", 13);
    allowed.insert("blue", 14);
    INPUT
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let mut game_parts = parts.next().unwrap().split(' ');
            let games_parts = parts.next().unwrap().split("; ");
            // extract game numeric ID
            game_parts.next();
            let game_id = game_parts.next().unwrap();
            let game_nr = game_id.parse::<u32>().unwrap();
            println!("Game {}", game_nr);
            // extract games
            let mut allowed_game = true;
            for game_str in games_parts {
                let cube_strings = game_str.split(", ");
                for cube_str in cube_strings {
                    let mut cube_parts = cube_str.split(' ');
                    let nr_cubes = cube_parts.next().unwrap().parse::<u32>().unwrap();
                    let cube_color = cube_parts.next().unwrap();
                    if nr_cubes > *allowed.get(cube_color).unwrap() {
                        allowed_game = false;
                        println!(
                            "Game {} is not allowed because it has {} {} cubes",
                            game_nr, nr_cubes, cube_color
                        );
                        break;
                    }
                }
            }
            (game_nr, allowed_game)
        })
        .filter(|(_, allowed_game)| *allowed_game)
        .map(|(game_nr, _)| game_nr)
        .sum::<u32>()
}

fn part2() -> u32 {
    INPUT
        .lines()
        .map(|line| {
            // map from color to minimum required items
            let mut max_map: HashMap<&str, u32> = HashMap::new();
            let mut parts = line.split(": ");
            let mut game_parts = parts.next().unwrap().split(' ');
            let games_parts = parts.next().unwrap().split("; ");
            // extract game numeric ID
            game_parts.next();
            let game_id = game_parts.next().unwrap();
            let game_nr = game_id.parse::<u32>().unwrap();
            println!("Game {}", game_nr);
            // extract games
            let mut allowed_game = true;
            for game_str in games_parts {
                let cube_strings = game_str.split(", ");
                for cube_str in cube_strings {
                    let mut cube_parts = cube_str.split(' ');
                    let nr_cubes = cube_parts.next().unwrap().parse::<u32>().unwrap();
                    let cube_color = cube_parts.next().unwrap();
                    match max_map.get(cube_color) {
                        Some(nr) => {
                            if nr_cubes > *nr {
                                max_map.insert(cube_color, nr_cubes);
                            }
                        }
                        None => {
                            max_map.insert(cube_color, nr_cubes);
                        }
                    }
                }
            }
            println!("fewest cubes {:?}", max_map);
            let mut power: u32 = 1;
            for cubes in max_map.values() {
                power *= cubes;
            }
            power
        })
        .sum::<u32>()
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
        assert_eq!(part1(), 2727);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 56580);
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
