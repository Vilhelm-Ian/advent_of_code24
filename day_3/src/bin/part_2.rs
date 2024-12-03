use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let result = solve(input);
    println!("{result}");
}

fn solve(input: &str) -> i32 {
    let input = format!("do(){input}don't()");
    let dont = Regex::new(r"do\(\).*?don\'t\(\)").unwrap();
    let re = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    let numbers = Regex::new(r"\d+").unwrap();
    let input: String = dont.find_iter(input.as_str()).map(|ele| ele.as_str()).collect();
    println!("{input}");
    re.find_iter(input.as_str())
        .map(|matched| {
            numbers
                .find_iter(matched.as_str())
                .map(|num| num.as_str().parse::<i32>().unwrap())
                .product::<i32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = solve(input);
        assert_eq!(result, 48);
    }
}
