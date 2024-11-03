#![feature(test)]
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    let solutions = [
        day01::main,
        day02::main,
        day03::main,
        day04::main,
        day05::main,
        day06::main,
        day07::main,
    ];
    for (day, solution) in solutions.iter().enumerate() {
        println!("------ Day {} ------", day + 1);
        solution()
    }
}
