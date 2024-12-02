fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

fn solve(input: &str) -> i32 {
    let mut result = 0;
    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();
    for line in lines {
        let ordering = line[1].cmp(&line[0]);
        let mut should_update = true;
        for index in 1..line.len() {
            let subtraction = line[index] - line[index - 1];
            let current_order = line[index].cmp(&line[index - 1]);
            if subtraction == 0 || subtraction.abs() > 3 || current_order != ordering {
                should_update = false;
                break;
            }
        }
        if should_update {
            result += 1;
        }
    }
    result
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
        assert_eq!(solve(input), 2);
    }
}
