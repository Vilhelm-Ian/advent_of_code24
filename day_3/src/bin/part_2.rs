use regex::Regex;

fn main() {
    let input = include_str!();
    println!("{solve(input)}");
}

fn solve(input: &str) -> i32 {
    let input = format!("do(){input}don't()");
    let dont = Regex::new(r"do\(\)(.|\n)*?don\'t\(\)").unwrap();
    let re = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    let numbers = Regex::new(r"\d+").unwrap();
    let input: String = dont.find_iter(input.as_str()).map(|ele| ele.as_str()).collect();
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
    
        #[test]
    fn it_works_2() {
      let input = "do()mul(1,1)don't()do()mul(2,2)don't()";
        let result = solve(input);
        assert_eq!(result, 5);
    }
            #[test]
    fn it_works_3() {
      let input = "mul(1,1)don't()do()mul(1,1)do()mul(2,2)";
        let result = solve(input);
        assert_eq!(result, 6);
    }
    #[test]
        fn it_works_4() {
      let input = "mul(1,1)don't()mul(1,1)do()mul(1,1)do()mul(2,2)";
        let result = solve(input);
        assert_eq!(result, 6);
    }
}
