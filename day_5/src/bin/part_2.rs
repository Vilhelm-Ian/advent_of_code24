use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result)
}

fn solve(input: &str) -> i32 {
    let mut result = 0;
    let (map, updates) = parse(input);
    for update in updates {
        for i in 0..update.len() - 1 {
            if let Some(rule) = map.get(&update[i]) {
                if rule.get(&update[i + 1]).is_none() {
                    break;
                }
            } else {
                break;
            }
            if i == update.len() - 2 {
                result += update[update.len() / 2];
            }
        }
    }
    result
}

fn fix_line(line: Vec<i32>, map: HashMap<i32, HashSet<i32>>) {
    for update in line {
        if let Some(rule) = map.get(&update[i]) {
            if rule.get(&update[i + 1]).is_none() {
                break;
            }
        } else {
            break;
        }
    }
}

fn parse(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut map = HashMap::new();
    let mut updates = vec![];
    let mut reading_rules = true;
    input.lines().for_each(|line| {
        if line.is_empty() {
            reading_rules = false;
        } else if reading_rules {
            let mut numbers = line.split("|");
            let first = numbers.next().unwrap().parse::<i32>().unwrap();
            let second = numbers.next().unwrap().parse::<i32>().unwrap();
            (*map.entry(first).or_insert(HashSet::new())).insert(second);
        } else {
            updates.push(
                line.split(",")
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect(),
            );
        }
    });
    (map, updates)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = solve(input);
        assert_eq!(result, 143);
    }
}
