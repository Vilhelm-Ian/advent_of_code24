use std::cmp::Ordering;

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

fn solve(input: &str) -> i32 {
    let lines = parse(input);
    lines
        .into_iter()
        .filter(|line| {
            if is_valid_line(line, true) {
                println!("o");
                true
            } else {
                println!("x");
                false
            }
        })
        .count() as i32
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_valid_line(line: &Vec<i32>, is_first_iteration: bool) -> bool {
    let ordering = line[1].cmp(&line[0]);
    for index in 1..line.len() {
        println!("{index}");
        if !is_valid_digits(line[index], line[index - 1], ordering)
            || (index == 1 && !handle_first_element(line))
        {
            if !is_first_iteration {
                return false;
            }
            println!("index {:?}", index);
            let mut first = line.clone();
            let mut second = line.clone();
            first.remove(index);
            second.remove(index - 1);
            println!("first {:?}", first);
            println!("second {:?}", second);
            return is_valid_line(&first, false) || is_valid_line(&second, false);
        }
    }
    true
}

fn is_valid_digits(first: i32, second: i32, ordering: Ordering) -> bool {
    let subtraction = first - second;
    let current_order = first.cmp(&second);
    if subtraction == 0 || subtraction.abs() > 3 || current_order != ordering {
        return false;
    }
    true
}

fn handle_first_element(line: &Vec<i32>) -> bool {
    let first = line[0];
    let second = line[1];
    let third = line[2];
    let subtraction = first - second;
    let current_order = first.cmp(&second);
    let ordering = second.cmp(&third);
    println!("line {:?}", line);
    if subtraction == 0 || subtraction.abs() > 3 || current_order != ordering {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(solve(input), 4);
    }
    #[test]
    fn last_digit() {
        let input: Vec<i32> = "43 44 47 49 49"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }
    #[test]
    fn two_mistakes() {
        let input: Vec<i32> = "86 89 90 93 93 95 94"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(!is_valid_line(&input, true));
    }
    #[test]
    fn one_mistake_four_difference() {
        let input: Vec<i32> = "6 7 9 11 13 14 18"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }
    #[test]
    fn two_mistakes_part_2() {
        let input: Vec<i32> = "86 86 88 89 86 88 89 93"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(!is_valid_line(&input, true));
    }
    #[test]
    fn three_repeating() {
        let input: Vec<i32> = "13 16 17 20 20 20"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(!is_valid_line(&input, true));
    }
    fn three_repeating_middle() {
        let input: Vec<i32> = "13 16 17 20 20 20 21"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }
    #[test]
    fn two_repeating_four_difference() {
        let input: Vec<i32> = "10 10 11 15 19"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(!is_valid_line(&input, true));
    }
    #[test]
    fn all_descending_but_one_four_difference() {
        let input: Vec<i32> = "25 24 18 17 14 8"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(!is_valid_line(&input, true));
    }
    #[test]
    fn first_double() {
        let input: Vec<i32> = "25 25 22 19 16"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }

    #[test]
    fn mid_dobule() {
        let input: Vec<i32> = "25 24 22 22 19 16"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }

    #[test]
    fn all_correct() {
        let input: Vec<i32> = "3 6 9 10 11"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }
    #[test]
    fn all_same() {
        let input: Vec<i32> = "3 3 3 3 3"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(!is_valid_line(&input, true));
    }

    #[test]
    fn last_bad() {
        let input: Vec<i32> = "1 2 3 4 9"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }

    #[test]
    fn second_to_last_bad() {
        let input: Vec<i32> = "1 2 3 9 4"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }

    #[test]
    fn first_bad() {
        let input: Vec<i32> = "9 2 3 4 5"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }

    #[test]
    fn first_repeat() {
        let input: Vec<i32> = "9 9 6 3 0"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }

    #[test]
    fn no_mistakes() {
        let input: Vec<i32> = "11 10 9 7 5 2"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }

    #[test]
    fn name_later() {
        let input: Vec<i32> = "3 6 9 8 11"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }

    #[test]
    fn name_later_2() {
        let input: Vec<i32> = "6 7 9 8 12 15"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }

    #[test]
    fn name_later_3() {
        let input: Vec<i32> = "10 12 15 13 14 18"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(!is_valid_line(&input, true));
    }

    #[test]
    fn name_later_5() {
        let input: Vec<i32> = "86 89 90 93 93 95 94"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(!is_valid_line(&input, true));
    }

    #[test]
    fn name_later_6() {
        let input: Vec<i32> = "48 45 47 50 53 56 57"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }

    #[test]
    fn name_later_6_but_simple() {
        let input: Vec<i32> = "45 47 50 53 56 57"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }

    #[test]
    fn name_later_6_but_more() {
        let input: Vec<i32> = "44 48 45 47 50 53 56 57"
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        assert!(is_valid_line(&input, true));
    }
}
