#![feature(test)]

use itertools::Itertools;

extern crate test;
const INPUT: &str = include_str!("../inputs/input13.txt");

const INPUT2: &str = "[[2]]\n[[6]]";

#[derive(Debug, PartialEq)]
enum Elem {
    Val(usize),
    List(Vec<Elem>),
}

fn parse(list_str: &str) -> (Option<Elem>, &str) {
    match list_str.chars().next().unwrap() {
        '[' => {
            let mut list: Vec<Elem> = Vec::new();
            let (mut element, mut rest) = parse(&list_str[1..]);
            while element.is_some() {
                list.push(element.unwrap());
                if !rest.starts_with(',') {
                    break;
                }
                (element, rest) = parse(&rest[1..]);
            }
            (Some(Elem::List(list)), &rest[1..])
        }
        ']' => (None, list_str),
        _ => {
            // parse integer part
            // find first non-digit
            let (pos, _) = list_str
                .chars()
                .find_position(|c| !c.is_ascii_digit())
                .unwrap();
            let val = list_str[0..pos].parse::<usize>().unwrap();
            (Some(Elem::Val(val)), &list_str[pos..])
        }
    }
}

fn compare(e1: &Elem, e2: &Elem) -> bool {
    match (e1, e2) {
        (Elem::Val(val1), Elem::Val(val2)) => {
            // println!("Compare [{}] vs [{}] is {}", val1, val2, val1 < val2);
            val1 < val2
        }
        (Elem::List(l1), Elem::List(l2)) => {
            for (index, l1_item) in l1.iter().enumerate() {
                let l2_item = l2.get(index);
                if l2_item.is_none() {
                    return false;
                }
                if compare(l1_item, l2_item.unwrap()) {
                    return true;
                }
                if compare(l2_item.unwrap(), l1_item) {
                    return false;
                }
            }
            if l1.len() < l2.len() {
                return true;
            }
            false
        }
        (Elem::Val(val1), Elem::List(_)) => {
            let list = vec![Elem::Val(*val1)];
            compare(&Elem::List(list), e2)
        }
        (Elem::List(_), Elem::Val(val2)) => {
            let list = vec![Elem::Val(*val2)];
            compare(e1, &Elem::List(list))
        }
    }
}
fn part1() -> usize {
    INPUT
        .split("\n\n")
        .enumerate()
        .map(|(i, pair)| {
            let index = i + 1;
            let mut pair_lines = pair.lines();
            let (left, _) = parse(pair_lines.next().unwrap());
            let (right, _) = parse(pair_lines.next().unwrap());
            let left = left.unwrap();
            let right = right.unwrap();
            // println!("left: {:?}", left);
            // println!("right: {:?}\n", right);
            (index, left, right)
        })
        .filter(|(_, left, right)| compare(left, right))
        .map(|(index, _, _)| index)
        .sum::<usize>()
}

fn comp(e1: &Elem, e2: &Elem) -> core::cmp::Ordering {
    if compare(e1, e2) {
        return core::cmp::Ordering::Less;
    };
    if compare(e2, e1) {
        return core::cmp::Ordering::Greater;
    }
    core::cmp::Ordering::Equal
}

fn part2() -> usize {
    let additional1 = INPUT2.lines().map(|line| {
        let (item, _) = parse(line);
        item.unwrap()
    });
    let mut additional2 = INPUT2.lines().map(|line| {
        let (item, _) = parse(line);
        item.unwrap()
    });
    let a1 = additional2.next().unwrap();
    let a2 = additional2.next().unwrap();
    INPUT
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (item, _) = parse(line);
            item.unwrap()
        })
        .chain(additional1)
        .sorted_by(comp)
        .enumerate()
        .filter(|(_, elem)| {
            if comp(&a1, elem) == core::cmp::Ordering::Equal {
                return true;
            }
            if comp(&a2, elem) == core::cmp::Ordering::Equal {
                return true;
            }
            false
        })
        .map(|(nth, _)| nth + 1)
        .product::<usize>()
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
        assert_eq!(part1(), 5393);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 26712);
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
