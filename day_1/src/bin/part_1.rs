fn main() {
    let input = include_str!("input.txt");
    let solution = solve(input);
    println!("{solution}");
}

fn solve(input: &str) -> i32 {
    let (mut a, mut b): (Vec<i32>, Vec<i32>) =
        input
            .lines()
            .fold((vec![], vec![]), |(mut a, mut b), line| {
                let mut numbers = line.split_whitespace();
                a.push(numbers.next().unwrap().parse().unwrap());
                b.push(numbers.next().unwrap().parse().unwrap());
                (a, b)
            });
    a.sort_unstable();
    b.sort_unstable();
    a.iter().zip(b).map(|(a, b)| (a - b).abs()).sum()
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
