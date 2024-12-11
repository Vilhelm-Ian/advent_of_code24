use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input, 75);
    println!("{result}");
}

pub fn solve(input: &str, times: usize) -> usize {
    let mut digits = parse(input);
    for _ in 0..times {
        let mut new_digits = HashMap::new();
        for (digit, digit_instances_num) in digits.iter() {
            let result = calculate(*digit);
            result
                .iter()
                .filter(|digit| digit.is_some())
                .for_each(|digit| {
                    *(new_digits.entry(digit.unwrap()).or_insert(0)) += digit_instances_num;
                });
        }
        digits = new_digits;
    }
    digits.values().sum()
}

fn parse(input: &str) -> HashMap<u64, usize> {
    let mut map = HashMap::new();
    input.trim().split(" ").for_each(|num| {
        *(map.entry(num.parse().unwrap()).or_insert(0)) += 1;
    });
    map
}

fn calculate(digit: u64) -> [Option<u64>; 2] {
    let mut result = [None, None];
    let log = (digit as f64).log10().floor() as u32;
    let pow = (log + 1) / 2;
    match digit {
        0 => {
            result[0] = Some(1);
        }
        _ => {
            if log % 2 == 0 {
                result[0] = Some(digit * 2024)
            } else {
                result[0] = Some(digit / 10_u64.pow(pow));
                result[1] = Some(digit % 10_u64.pow(pow));
            }
        }
    };
    result
}

fn count_stones(digits: &HashMap<u64, usize>) -> usize {
    let mut result = 0;
    for digit_instances_num in digits.values() {
        result += digit_instances_num;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_test() {
        let input = "125 17";
        let result = solve(input, 6);
        assert_eq!(result, 22);
    }
    #[test]
    fn second_test() {
        let input = "125 17";
        let result = solve(input, 25);
        assert_eq!(result, 55312);
    }
}
