# Advent of Code 2023
My [Advent of Code 2023](https://adventofcode.com) solutions in the Rust programming language.

This year I only had time to work on puzzles 1-5 (day01-dayo05).
The excecution time (measured with cargo bench) for all 5 part1 and part2 solutions is 1.3 milliseconds.

Structure of solutions modeled after https://github.com/ahmadkaouk/advent-of-code-2021

## Usage
```sh
# Run all the days
cargo run --release

# Test a specific day
cargo test --bin day01

#Run a specific day
cargo run --release --bin day01
```
## Benchmark
### Run

```sh
cargo bench
```



### Timing

|                       | Problem                                            | Part 1   | Part 2   |   
|-----------------------|----------------------------------------------------|----------|----------|
| [Day 1](src/day01.rs) | [Problem 1](https://adventofcode.com/2023/day/1)   |   0.071 ms |   0.113 ms | 
| [Day 2](src/day02.rs) | [Problem 2](https://adventofcode.com/2023/day/2)   |   0.090 ms |   0.167 ms |   
| [Day 3](src/day03.rs) | [Problem 3](https://adventofcode.com/2023/day/3)   |   0.034 ms |   0.235 ms | 
| [Day 4](src/day04.rs) | [Problem 4](https://adventofcode.com/2023/day/4)   |   0.218 ms |   0.227 ms | 
| [Day 5](src/day05.rs) | [Problem 5](https://adventofcode.com/2023/day/5)   |   0.041 ms |   0.057 ms | 
| [Day 6](src/day06.rs) | [Problem 6](https://adventofcode.com/2023/day/6)   |     623 ns |     755 ns | 
| [Day 7](src/day07.rs) | [Problem 7](https://adventofcode.com/2023/day/7)   |   0.148 ms |   0.164 ms | 


> The benchmarks are measured with the unstable bench feature of Rust using my Macbook Pro 13' mid 2019
