use regex::Regex;

fn main() {
    let input = "";
    let result = solve(input);
    println!("{result}");
}

fn parse(input: &str) -> Vec<Vec<Vec<i128>>> {
    let re = Regex::new(r"\d+").unwrap();
    input
        .split("\n\n")
        .map(|game| {
            game.lines()
                .enumerate()
                .map(|(i, line)| {
                    re.find_iter(line)
                        .map(|number| {
                            if i == 2 {
                                number.as_str().parse::<i128>().unwrap() + 10000000000000
                            } else {
                                number.as_str().parse::<i128>().unwrap()
                            }
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn solve(input: &str) -> i128 {
    let games = parse(input);
    let mut result = 0;
    for game in games {
        let x1 = game[0][0];
        let y1 = game[0][1];
        let x2 = game[1][0];
        let y2 = game[1][1];
        let x3 = game[2][0];
        let y3 = game[2][1];
        let m = (x1 * y3 - x3 * y1) / (-x2 * y1 + x1 * y2);
        let n = (x3 - m * x2) / x1;
        if n * x1 + m * x2 == x3 && n * y1 + m * y2 == y3 {
            result += m + n * 3
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    #[test]
    fn part_1_2() {
        let result = solve(INPUT);
        assert_eq!(280, result);
    }
}
