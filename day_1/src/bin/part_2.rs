use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let solution = solution(input);
    println!("{solution}");
}

fn solution(input: &str) -> i32 {
    let mut a = vec![];
    let mut b = HashMap::new();
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>()
        .into_iter()
        .for_each(|nums| {
            a.push(nums[0]);
            let temp = b.entry(nums[1]).or_insert(0);
            *temp += 1;
        });
    a.sort();
    let result = a.iter().map(|number| number * b.get(number).unwrap_or(&0));
    result.sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(solution(input), 31);
    }
}
