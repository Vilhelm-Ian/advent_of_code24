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

fn is_valid(target: i64, mut numbers: &Vec<i64>) -> bool {
    // let mut numbers_clone = numbers.clone();
    // println!("target {target}, numbers {:?}", numbers);
    // for number in numbers.iter().rev() {
    if numbers.is_empty() {
        if target == 1 || target == 0 {
            return true;
        }
        return false;
    }
    let mut numbers = numbers.clone();
    let number = numbers.pop().unwrap();
    if target % number == 0 && is_valid(target / number, &numbers) {
        return true;
    }

    if target - number > 0 && is_valid(target - number, &numbers) {
        return true;
    }

    if let Some(target) = de_concate(target, number) {
        if is_valid(target, &numbers) {
            return true;
        }
    }

    // }
    false
}

fn de_concate(result: i64, b: i64) -> Option<i64> {
    let result = result.to_string();
    let b_str = b.to_string();
    if (result.len() as i32 - b_str.len() as i32) < 0 {
        return None;
    }
    let (a, b_og) = result.split_at(result.len() - b_str.len());
    if let Ok(b_og) = b_og.parse::<i64>() {
        if b != b_og {
            return None;
        }
    } else {
        return None;
    }
    if let Ok(a) = a.parse::<i64>() {
        return Some(a);
    }
    None
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
        assert_eq!(result, 11387);
    }

    #[test]
    fn part_1_1() {
        let result = is_valid(190, &vec![10, 19]);
        assert!(result);
    }

    #[test]
    fn part_1_2() {
        let result = is_valid(3267, &vec![81, 40, 27]);
        assert!(result);
    }

    #[test]
    fn part_1_3() {
        let result = is_valid(292, &vec![11, 6, 16, 20]);
        assert!(result);
    }

    #[test]
    fn part_2_1() {
        let result = is_valid(156, &vec![15, 6]);
        assert!(result);
    }

    #[test]
    fn part_2_2() {
        let result = is_valid(7290, &vec![6, 8, 6, 15]);
        assert!(result);
    }

    #[test]
    fn part_2_3() {
        let result = is_valid(192, &vec![17, 8, 14]);
        assert!(result);
    }

    #[test]
    fn de_concate_test() {
        let result = de_concate(3267, 67).unwrap();
        assert_eq!(result, 32);
    }

    #[test]
    fn de_concate_test_2() {
        let result = de_concate(178, 8).unwrap();
        assert_eq!(result, 17);
    }
}
