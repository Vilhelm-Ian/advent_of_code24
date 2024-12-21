use std::collections::{HashSet, VecDeque};
use std::iter::Iterator;
use std::ops::Deref;

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

type Grid = Vec<Vec<char>>;
type Cordinate = [usize; 2];
type Direction = [i32; 2];

const NUMERIC_PAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve(input: &str) -> i32 {
    let codes = parse(input);
    find_shortest_numeric_pad(&codes[0]);
    0
}

//029A

fn find_shortest_numeric_pad(code: &Vec<char>) -> Vec<char> {
    let start = [3, 2];
    let directions: Vec<[i32; 2]> = vec![[1, 0], [0, 1], [-1, 0], [0, -1]];
    let mut que = VecDeque::new();
    que.push_front((start, vec![]));
    let mut visited = HashSet::new();
    let mut code = code.iter();
    let mut target = code.next();
    let mut result = vec![];
    while let Some((current, mut path)) = que.pop_back() {
        if visited.contains(&current) {
            continue;
        }
        // println!("{:?}", target);
        // println!("current {:?}", current);
        visited.insert(current);
        if NUMERIC_PAD[current[0]][current[1]] == ' ' {
            continue;
        }
        if let Some(cur_target) = target {
            if *cur_target == NUMERIC_PAD[current[0]][current[1]] {
                visited = HashSet::new();
                que = VecDeque::new();
                target = code.next();
                path.push([0, 0]);
            }
            let mut pt = path.iter().map(direction_to_arrow).collect::<String>();
            println!("{:?}", pt);
        } else {
            println!("{:?}", path);
            break;
        }
        directions
            .iter()
            .map(|direction| (update_cordinate(current, *direction), direction))
            .filter(|(current, _)| {
                current[0] < NUMERIC_PAD.len() && current[1] < NUMERIC_PAD[0].len()
            })
            .for_each(|(current, direction)| {
                let mut path = path.clone();
                path.push(*direction);
                que.push_front((current, path))
            });
    }
    result
}

fn update_cordinate(current_cordinate: Cordinate, direction: Direction) -> Cordinate {
    [
        (current_cordinate[0] as i32 + direction[0]) as usize,
        (current_cordinate[1] as i32 + direction[1]) as usize,
    ]
}

fn direction_to_arrow(direction: &Direction) -> char {
    match direction {
        [0, 1] => '>',
        [0, -1] => '<',
        [-1, 0] => '^',
        [1, 0] => 'v',
        [0, 0] => 'A',
        _ => panic!("not vaalid diirection"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "029A";
    #[test]
    fn it_works() {
        let result = solve(INPUT);
        assert_eq!(result, 4);
    }
}
