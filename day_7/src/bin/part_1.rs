use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Perumtation {
    value: i64,
    seen: Vec<usize>,
}

impl Perumtation {
    fn new(value: i64, seen: Vec<usize>) -> Self {
        Perumtation { value, seen }
    }
    fn update(&mut self, value: i64, index: usize) {
        self.value = value;
        self.seen.push(index);
        self.seen.sort();
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{result}");
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    let mut result = vec![];
    for line in input.lines() {
        let mut line_numbers = vec![];
        for (i, num) in line.split(" ").enumerate() {
            if i == 0 {
                line_numbers.push(num.split(":").next().unwrap().parse().unwrap())
            } else {
                line_numbers.push(num.parse().unwrap())
            }
        }
        result.push(line_numbers);
    }
    result
}

fn solve(input: &str) -> i64 {
    let input = parse(input);
    let mut result = 0;
    input
        .par_iter()
        .filter(|line| {
            let mut line_clone = (*line).clone();
            let first = line_clone.remove(0);
            is_valid(first, &line_clone)
        })
        .map(|line| line[0])
        .sum()
}

fn is_valid(mut target: i64, mut numbers: &Vec<i64>) -> bool {
    let mut numbers_clone = numbers.clone();
    for number in numbers.iter().rev() {
        numbers_clone.pop();
        if target % number == 0 {
            let parallel_dimension = target - number;
            target /= number;
            if parallel_dimension > 0 {
                println!("{parallel_dimension} {:?}", numbers);
                let parallel = is_valid(parallel_dimension, &numbers_clone);
                if parallel {
                    return true;
                }
            }
        } else if target - number > 0 {
            target -= number
        } else {
            return false;
        }
    }
    target == 1 || target == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    #[test]
    fn it_works() {
        let result = solve(INPUT);
        assert_eq!(result, 3749);
    }
    #[test]
    fn part_1_2() {
        let result = is_valid(3267, &vec![81, 40, 27]);
        assert!(result);
    }
}
