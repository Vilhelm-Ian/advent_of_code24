use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{}", result.1);
}

pub fn solve(input: &str) -> ([i32; 4], u64) {
    let mut numbers = parse(input);
    let mut res = HashMap::new();
    numbers.into_iter().for_each(|mut num| {
        let mut que = [0; 4];
        let mut prev = 0;
        let mut map = HashMap::new();
        for i in 0..2000 {
            let next = next_number(num);
            if i == 0 {
                prev = num;
                continue;
            }
            que = [que[1], que[2], que[3], num as i32 % 10 - prev as i32 % 10];
            let mut m = [0; 4];
            if i > 5 {
                m = que;
            }
            if i > 5 && !map.contains_key(&m) {
                *res.entry(m).or_insert(0) += num % 10;
                map.insert(m, num % 10);
            }
            prev = num;

            num = next;
        }
    });
    res.into_iter().max_by_key(|(_, v)| *v).unwrap()
}

fn next_number(mut number: u64) -> u64 {
    number = first(number);
    number = second(number);
    number = third(number);
    number
}

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|num| num.trim().parse::<u64>().unwrap())
        .collect()
}

fn first(number: u64) -> u64 {
    let mut cur_number = number * 2u64.pow(6);
    cur_number ^= number;
    cur_number %= 2u64.pow(24);
    cur_number
}

fn second(number: u64) -> u64 {
    let mut cur_number = number / 2u64.pow(5);
    cur_number ^= number;
    cur_number %= 2u64.pow(24);
    cur_number
}

fn third(number: u64) -> u64 {
    let mut cur_number = number * 2u64.pow(11);
    cur_number ^= number;
    cur_number %= 2u64.pow(24);
    cur_number
}

fn prune(number: u64) -> u64 {
    number ^ 2u64.pow(24)
}

fn mix(number: u64, number_2: u64) -> u64 {
    number ^ number_2
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "1
2
3
2024";
    use super::*;
    #[test]
    fn part_2() {
        let result = solve(INPUT);
        assert_eq!(23, result.1);
        assert_eq!([-2, 1, -1, 3], result.0);
    }
}
