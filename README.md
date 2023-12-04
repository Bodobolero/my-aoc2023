# Advent of Code 2023
My [Advent of Code 2023](https://adventofcode.com) solutions in the Rust programming language.

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

Days 5-25 are placeholders copied from aoc 2022

> The benchmarks are measured with the unstable bench feature of Rust using my Macbook Pro 13' mid 2019
