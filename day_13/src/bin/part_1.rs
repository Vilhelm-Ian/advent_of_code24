use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

fn solve(input: &str) -> i32 {
    let games = parse(input);
    let mut result = 0;
    for game in games {
        println!("{:?}", game[0]);
        println!("{:?}", game[1]);
        println!("{:?}", game[2]);
        let [x1, y1] = [game[0][0], game[0][1]];
        let [x2, y2] = [game[1][0], game[1][1]];
        let [x3, y3] = [game[2][0], game[2][1]];
        let (a, b) = find_combination(x1, y1, x2, y2, x3, y3);
        println!("a {a} b {b}");
        result += a * 3 + b;
    }
    result
}

fn parse(input: &str) -> Vec<Vec<Vec<i32>>> {
    let re = Regex::new(r"\d+").unwrap();
    let games: Vec<Vec<Vec<i32>>> = input
        .split("\n\n")
        .map(|game| {
            game.lines()
                .map(|line| {
                    re.find_iter(line)
                        .map(|number| number.as_str().parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    games
}

fn find_combination(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> (i32, i32) {
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let difference_first_cordinates: i32 = x1 - y1;
    let difference_second_cordinates: i32 = x2 - y2;
    let difference_result_cordinates: i32 = x3 - y3;
    loop {
        n += 1;
        if n > 100 || m > 100 {
            return (0, 0);
        }
        if difference_first_cordinates == 0
            || difference_second_cordinates == 0
            || difference_result_cordinates == 0
        {
            if (x3 - x2 * n) % x1 == 0 {
                m = (x3 - x2 * n) / x1;
            };
        } else if (difference_result_cordinates - n * difference_second_cordinates)
            % difference_first_cordinates
            == 0
        {
            m = (difference_result_cordinates - n * difference_second_cordinates)
                / difference_first_cordinates;
        } else {
            continue;
        }
        if m * x1 + n * x2 == x3 && m * y1 + n * y2 == y3 {
            break;
        }
        if m * x1 + n * x2 > x3 && m * y1 + n * y2 > y3 {
            return (0, 0);
        }
    }
    (m, n)
}

#[cfg(test)]
mod tests {
    pub const TEST: &str = "Button A: X+94, Y+34
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
    use super::*;
    #[test]
    fn test_2() {
        let result = solve(TEST);
        assert_eq!(result, 480);
    }
    #[test]
    fn test_3() {
        let result = find_combination(74, 74, 25, 97, 6236, 10556);
        assert_eq!(result, (64, 60));
    }
}
