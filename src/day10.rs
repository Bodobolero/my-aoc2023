#![feature(test)]

extern crate test;
const INPUT: &str = include_str!("../inputs/input10.txt");

fn check_and_add_sum_then_inc_cycle(sum: &mut i32, x: i32, cycle: &mut i32) {
    if *cycle >= 20 && (*cycle - 20) % 40 == 0 {
        *sum += *cycle * x;
        println!(
            "cycle {} adder {} product {} sum {}",
            *cycle,
            x,
            *cycle * x,
            *sum
        );
    }
    *cycle += 1;
}

fn part1() -> i32 {
    let mut x: i32 = 1;
    let mut cycle: i32 = 1;
    let mut adder: i32;
    let mut sum: i32 = 0;
    for line in INPUT.lines() {
        let mut parts = line.split(" ");
        match parts.next().unwrap() {
            "addx" => {
                check_and_add_sum_then_inc_cycle(&mut sum, x, &mut cycle);
                check_and_add_sum_then_inc_cycle(&mut sum, x, &mut cycle);
                adder = parts.next().unwrap().parse::<i32>().unwrap();
                x += adder;
            }
            "noop" => {
                check_and_add_sum_then_inc_cycle(&mut sum, x, &mut cycle);
            }
            _ => panic!("unexpected command"),
        }
    }
    sum
}

fn printcrt(crt: &Vec<Vec<u8>>) {
    for i in 0..6 {
        for j in 0..40 {
            print!("{}", crt[i][j] as char);
        }
        println!("");
    }
}

fn process_cycle(x: i32, cycle: &mut i32, crt: &mut Vec<Vec<u8>>) {
    // println!("cycle {} adder {}", *cycle, x);
    // printcrt(crt);

    // process sprite
    let (row, column) = (((*cycle - 1) / 40) as usize, ((*cycle - 1) % 40) as usize);
    // is sprite positioned over current position "column" ?
    if column as i32 >= (x - 1) && column as i32 <= (x + 1) {
        crt[row][column] = b'#';
    }

    *cycle += 1;
}

fn part2() -> Vec<Vec<u8>> {
    let mut x: i32 = 1;
    let mut cycle: i32 = 1;
    let mut adder: i32;
    let mut crt = vec![vec![b'.'; 40]; 6];
    for line in INPUT.lines() {
        let mut parts = line.split(" ");
        match parts.next().unwrap() {
            "addx" => {
                process_cycle(x, &mut cycle, &mut crt);
                process_cycle(x, &mut cycle, &mut crt);
                adder = parts.next().unwrap().parse::<i32>().unwrap();
                x += adder;
            }
            "noop" => {
                process_cycle(x, &mut cycle, &mut crt);
            }
            _ => panic!("unexpected command"),
        }
    }
    crt
}

pub fn main() {
    println!("Part 1: Answer {}", part1());
    println!("Part 2: Answer ");
    printcrt(&part2());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 12540);
    }
    #[test]
    fn part2_test() {
        let expected = vec![
            b"####.####..##..####.####.#....#..#.####.",
            b"#....#....#..#....#.#....#....#..#.#....",
            b"###..###..#......#..###..#....####.###..",
            b"#....#....#.....#...#....#....#..#.#....",
            b"#....#....#..#.#....#....#....#..#.#....",
            b"#....####..##..####.####.####.#..#.####.",
        ];
        let result = part2();
        for i in 0..6 {
            for j in 0..40 {
                assert!(expected[i][j] == result[i][j]);
            }
        }
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
