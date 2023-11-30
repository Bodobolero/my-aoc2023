# Advent of Code 2022
My [Advent of Code 2022](https://adventofcode.com) solutions in the Rust programming language.

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
| [Day 1](src/day01.rs) | [Problem 1](https://adventofcode.com/2023/day/1)   |   x.xxx ms |   x.xxx ms |   

Days 2-25 are placeholders copied from aoc 2022

> The benchmarks are measured with the unstable bench feature of Rust using my Macbook Pro 13' mid 2019
