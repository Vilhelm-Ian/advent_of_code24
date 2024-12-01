fn main() {
    let input = include_str!("input.txt");
    let solution = solve(input);
    println!("{solution}");
}

fn solve(input: &str) -> i32 {
    let mut a = vec![];
    let mut b = vec![];
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>()
        .iter()
        .for_each(|nums| {
            a.push(nums[0]);
            b.push(nums[1]);
        });
    a.sort();
    b.sort();
    let mut result = 0;
    for i in 0..a.len() {
        result += (a[i] - b[i]).abs();
    }
    result
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
        assert_eq!(solve(input), 11);
    }
}
