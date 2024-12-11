fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input, 25);
    println!("{result}");
}

pub fn solve(input: &str, times: usize) -> usize {
    let mut digits = parse(input);
    for _ in 0..times {
        let len = digits.len();
        for i in 0..len {
            let [first, second] = calculate(&digits, i);
            digits[i] = first.unwrap();
            if let Some(digit) = second {
                digits.push(digit)
            }
        }
    }
    digits.len()
}

fn parse(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(" ")
        .map(|num| num.parse().unwrap())
        .collect()
}

fn calculate(digits: &Vec<u64>, index: usize) -> [Option<u64>; 2] {
    let mut result = [None, None];
    let log = (digits[index] as f64).log10().floor() as u32;
    let pow = (log + 1) / 2;
    match digits[index] {
        0 => {
            result[0] = Some(1);
        }
        _ => {
            if log % 2 == 0 {
                result[0] = Some(digits[index] * 2024)
            } else {
                result[0] = Some(digits[index] / 10_u64.pow(pow));
                result[1] = Some(digits[index] % 10_u64.pow(pow));
            }
        }
    };
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
