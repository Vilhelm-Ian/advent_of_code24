use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

fn main() {
 let input = include_str!("./input.txt");
 let result = solve(input);
 println!("{result}");
}

fn solve(input: &str) -> i32 {
    let mut result = 0;
    let (map, updates) = parse(input);
    let mut wronged = vec![];
    for update in updates {
        for i in 0..update.len() - 1 {
            if let Some(rule) = map.get(&update[i]) {
                if rule.get(&update[i + 1]).is_none() {
                    wronged.push(update);
                    break;
                }
            } else {
                wronged.push(update);
                break;
            }
        }
    }
    wronged.iter().map(|line| sort_removed(line.clone(),map.clone())).map(|line| fix_line(line.clone(), map.clone())).map(|line| line[line.len() /2]).sum()
}

fn remove_wrong(mut update: Vec<i32>, map: HashMap<i32, HashSet<i32>>) -> (Vec<i32>, Vec<i32>) {
    let mut removed = vec![];
    let mut i = 0;
    while i < update.len() - 1 {
        if let Some(rule) = map.get(&update[i]) {
            if rule.get(&update[i + 1]).is_none() {
                removed.push(update.remove(i));
                continue;
            }
        } else {
               removed.push(update.remove(i));
               continue;
        }
        i+= 1;
    }
    return (update, removed)
}

fn sort_removed(mut removed: Vec<i32>, map: HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    removed.sort_by(move |a,b| {
        if let Some(num1) = map.get(a) {
            if num1.get(b).is_some() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Less
        }
    } );
    removed
}

fn fix_line(update: Vec<i32>, map: HashMap<i32, HashSet<i32>>)-> Vec<i32> {
    let (mut result, removed) = remove_wrong(update, map.clone());
    for wronged in removed {
        for i in 0..result.len() {
        let num = result[i];
            if let Some(name_later) = map.get(&wronged) {
                if name_later.get(&num).is_some() {
                    result.insert(i, wronged);
                    break;
                }
            }
            if i == result.len() - 1 {
                result.push(wronged);
            }
        }
    }
    result
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
        assert_eq!(result, 123);
    }
    
    #[test]
    fn fix_1() {
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
    let (map, lines) = parse(input);
    let result = fix_line(lines[3].clone(), map);
    assert_eq!(result, vec![97,75,47,61,53]);
    }
    
    #[test]
    fn fix_2() {
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
    let (map, lines) = parse(input);
    let result = fix_line(lines[4].clone(), map);
    assert_eq!(result, vec![61,29,13]);
    }
    
    #[test]
    fn fix_3() {
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
    let (map, lines) = parse(input);
    let result = fix_line(lines[5].clone(), map);
    assert_eq!(result, vec![97,75,47,29,13]);
    }
}
