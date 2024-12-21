use std::collections::{HashMap, HashSet, VecDeque};
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
    let mut start = [3, 2];
    let directions: Vec<[i32; 2]> = vec![[1, 0], [0, 1], [-1, 0], [0, -1]];
    // let mut visited = HashMap::new();
    let mut code = code.iter();
    // let mut result = vec![];
    let mut name_later = vec![];
    for code in code {
        let target = code_to_index(*code);
        let rez = depth_first_search(
            target,
            vec![],
            start,
            HashSet::new(),
            manhatan_distance(target, start) + 1,
            0,
        );
        // println!("{:?}", rez.flatten());
        // println!(
        //     "{:?}",
        //     rez.iter()
        //         .map(|name| name.chunks(2).collect::<Vec<&[Cordinate]>>())
        //         .collect::<Vec<Vec<&[Cordinate]>>>()
        // );
        let a = rez
            .into_iter()
            .map(|h| {
                h.windows(2)
                    .map(|w| direction_from_distances(w[1], w[0]))
                    .map(|dir| direction_to_arrow(&dir))
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();
        name_later.push(a);
        // rez.iter().map(||)
        start = target;
        // generate_combinations(, r, , )
    }
    // println!("{:?}", name_later);
    let h = generate_combinations(&name_later[0][0], &name_later[1..], vec![]);
    for z in h {
        println!("{:?}", z);
    }
    vec![]
}

fn depth_first_search(
    target: Cordinate,
    mut path: Vec<Cordinate>,
    current_cordinate: Cordinate,
    mut seen: HashSet<Cordinate>,
    distance: usize,
    steps: usize,
) -> Vec<Vec<Cordinate>> {
    path.push(current_cordinate);
    for index in current_cordinate.iter() {
        if *index > 3 {
            return vec![];
        }
    }
    if seen.contains(&current_cordinate) {
        return vec![];
    }
    if path.len() > distance || current_cordinate == [3, 0] {
        return vec![];
    }
    if path.len() == distance && target == current_cordinate {
        return vec![path];
    }
    seen.insert(current_cordinate);
    let mut result = vec![];
    let directions: Vec<[i32; 2]> = vec![[1, 0], [0, 1], [-1, 0], [0, -1]];
    for directiton in directions {
        let new_cordinate = update_cordinate(current_cordinate, directiton);
        for name_later in depth_first_search(
            target,
            path.clone(),
            new_cordinate,
            seen.clone(),
            distance,
            steps + 1,
        ) {
            // if !result.contains(&name_later) {
            result.push(name_later);
            // }
        }
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

fn manhatan_distance(a: Cordinate, b: Cordinate) -> usize {
    (a[0] as i32 - b[0] as i32).abs() as usize + (a[1] as i32 - b[1] as i32).abs() as usize
}

fn code_to_index(code: char) -> Cordinate {
    match code {
        '9' => [0, 2],
        '8' => [0, 1],
        '7' => [0, 0],
        '6' => [1, 2],
        '5' => [1, 1],
        '4' => [1, 0],
        '3' => [2, 2],
        '2' => [2, 1],
        '1' => [2, 0],
        'A' => [3, 2],
        '0' => [3, 1],
        _ => panic!("not valid code"),
    }
}

fn generate_combinations<T>(current: &T, following: &[Vec<T>], mut result: Vec<T>) -> Vec<Vec<T>>
where
    T: std::fmt::Debug + Clone,
{
    result.push(current.clone());
    // println!("{:?}", result);
    let mut back = vec![];
    if following.is_empty() {
        return vec![result];
    }
    for name_later in &following[0] {
        let rt = generate_combinations(name_later, &following[1..], result.clone());
        back.extend(rt)
    }
    back
}

fn direction_from_distances(a: Cordinate, b: Cordinate) -> Direction {
    [a[0] as i32 - b[0] as i32, a[1] as i32 - b[1] as i32]
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
    #[test]
    fn dfs() {
        let mut result = depth_first_search([1, 1], vec![], [2, 2], HashSet::new(), 3, 0);
        let result_2 = depth_first_search([0, 0], vec![], [1, 1], HashSet::new(), 3, 0);
        result.iter_mut().for_each(|path| {
            path.pop();
        });
        for path_1 in result.iter() {
            for path_2 in result_2.iter() {
                println!(
                    "{:?}",
                    path_1
                        .iter()
                        .chain(path_2.iter())
                        .cloned()
                        .collect::<Vec<Cordinate>>()
                );
            }
        }
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn combinatins_1() {
        let following = vec![vec![0], vec![1, 2, 3], vec![1, 2]];
        let res = generate_combinations(&following[0][0], &following[1..], vec![]);
        println!("{:?}", res);
        assert_eq!(1, 2);
    }
}
