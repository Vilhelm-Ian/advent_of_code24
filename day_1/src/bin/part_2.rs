use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let solution = solution(input);
    println!("{solution}");
}

fn solution(input: &str) -> i32 {
    let mut a = vec![];
    let mut b = HashMap::new();
    input.lines().for_each(|line| {
        let mut numbers = line.split_whitespace();
        a.push(numbers.next().unwrap().parse::<i32>().unwrap());
        let second_number = numbers.next().unwrap().parse::<i32>().unwrap();
        *(b.entry(second_number).or_insert(0)) += 1;
    });
    a.iter()
        .map(|number| number * b.get(number).unwrap_or(&0))
        .sum()
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
