use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>, usize) {
    input.lines().fold(
        (vec![], Vec::new(), 0),
        |(mut towels, mut designs, mut max), line| {
            if designs.is_empty() {
                line.split(',').for_each(|design| {
                    designs.push(design.trim());
                });
                max = designs.iter().max_by_key(|m| m.len()).unwrap().len();
            } else {
                towels.push(line);
            }
            (towels, designs, max)
        },
    )
}
// from my input I have singlee letter, w, u, r, g, missing b.
// double letter bb, bw, bu, bg, missing br
// triple letter brb, brg, brr, brw, bru,

fn solve(input: &str) -> i32 {
    let (towels, mut designs, max) = parse(input);
    let mut designs_sets = HashSet::new();
    for design in designs {
        designs_sets.insert(design);
    }
    let designs_sets_reduced = remove_redundant_designs(designs_sets);
    let mut valid_towels = vec![];
    for towel in towels.iter().skip(1) {
        let combinations = get_combinations_2(towel, &designs_sets_reduced);
        if !combinations.is_empty() {
            valid_towels.push(combinations);
        }
    }

    0
}

fn find_variations(combination: Vec<&str>, designs: HashSet<&str>) -> i32 {
    let mut result = 0;
    for i in 0..combination.len() {}

    result
}

fn get_combinations(target: i32, elements: &Vec<i32>) -> Vec<Vec<i32>> {
    if target == 0 {
        return vec![vec![]];
    }
    let mut result = vec![];
    for element in elements {
        if *element <= target {
            let combinations = get_combinations(target - element, elements);
            for combination in combinations {
                let mut n = vec![*element];
                n.extend_from_slice(&combination);
                result.push(n);
            }
        }
    }
    result
}

fn get_combinations_2<'a>(target: &str, elements: &HashSet<&'a str>) -> Vec<Vec<&'a str>> {
    if target == "" {
        return vec![vec![]];
    }
    let mut result = vec![];
    for element in elements {
        if !result.is_empty() {
            break;
        }
        if target.starts_with(element) {
            let combinations = get_combinations_2(&target[element.len()..], elements);
            for combination in combinations {
                let mut n = vec![*element];
                n.extend_from_slice(&combination);
                result.push(n);
            }
        }
    }
    result
}

fn remove_redundant_designs(mut designs: HashSet<&str>) -> HashSet<&str> {
    let mut designs_vec: Vec<&str> = designs.clone().into_iter().collect();
    for design in designs_vec {
        let combinations =
            get_combinations(design.len() as i32, &(1..design.len() as i32).collect());
        for combination in combinations {
            let mut i = 0;
            let mut should_remove = true;
            for index in combination {
                if i + index as usize > design.len() {
                    break;
                }
                if !designs.contains(&design[i..i + index as usize]) {
                    should_remove = false;
                    break;
                }
                i += index as usize;
            }
            if should_remove {
                designs.remove(design);
                break;
            }
        }
    }
    designs
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    #[test]
    fn it_works() {
        let result = solve(INPUT);
        assert_eq!(result, 6);
    }
}
