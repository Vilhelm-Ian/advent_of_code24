use std::collections::{BinaryHeap, HashMap, VecDeque};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    // println!("{result}");
}

pub fn solve(input: &str) -> u64 {
    let mut numbers = parse(input);
    // let map = HashMap::new();
    numbers
        .into_iter()
        .map(|mut num| {
            (0..2000).for_each(|_| num = next_number(num));
            num
        })
        .sum()
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

//2^6       1000000
fn first(number: u64) -> u64 {
    // println!("{:024b} og", number);
    let mut cur_number = number * 2u64.pow(6);
    // println!("{:024b} shifting <", cur_number);
    cur_number ^= number;
    // println!("{:024b} xor", cur_number);
    cur_number %= 2u64.pow(24);
    // println!("{:024b} modulos", cur_number);
    cur_number
}

//2^5        100000
fn second(number: u64) -> u64 {
    let mut cur_number = number / 2u64.pow(5);
    // println!("{:024b} shifting >", cur_number);
    cur_number ^= number;
    // println!("{:024b} xor", cur_number);
    cur_number %= 2u64.pow(24);
    // println!("{:024b} modulos", cur_number);
    cur_number
}

//2^11 100000000000
fn third(number: u64) -> u64 {
    let mut cur_number = number * 2u64.pow(11);
    // println!("{:024b} shifting <", cur_number);
    cur_number ^= number;
    // println!("{:024b} xora", cur_number);
    cur_number %= 2u64.pow(24);
    // println!("{:024b} modulos", cur_number);
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
10
100
2024";
    use super::*;
    #[test]
    fn it_works() {
        let result = solve(INPUT);
        assert_eq!(37327623, result);
    }
}
