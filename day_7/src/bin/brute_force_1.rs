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

fn is_valid(target: i64, numbers: &Vec<i64>) -> bool {
    let mut result = numbers.clone();
    let mut permutations = HashSet::new();
    result.iter().enumerate().for_each(|(index, value)| {
        permutations.insert(Perumtation::new(*value, vec![index]));
    });
    for (i, number) in numbers.iter().enumerate() {
        let mut new_permutation = permutations.clone();
        for permutation in permutations.iter() {
            if permutation.seen.contains(&i) {
                continue;
            }
            let product = number * permutation.value;
            let sum = number + permutation.value;
            if sum <= target {
                let mut name_later_1 = permutation.clone();
                name_later_1.update(sum, i);
                if sum == target && name_later_1.seen.len() == numbers.len() {
                    return true;
                }
                new_permutation.insert(name_later_1);
            }
            if product <= target {
                let mut name_later_2 = permutation.clone();
                name_later_2.update(product, i);
                if product == target && name_later_2.seen.len() == numbers.len() {
                    return true;
                }
                new_permutation.insert(name_later_2);
            }
        }
        permutations = new_permutation;
    }
    false
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
    fn fiveofive() {
        let result = is_valid(505, &vec![55, 1, 9]);
        assert!(!result);
    }
}
