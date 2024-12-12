use std::{
    any::Any,
    collections::{HashMap, HashSet},
    env::current_dir,
};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

fn solve(input: &str) -> usize {
    let grid = parse(input);
    let mut seen = HashMap::new();
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut edges = HashSet::new();
            let mut current_area = HashMap::new();
            if seen.contains_key(&[y, x]) {
                continue;
            }
            traverse([y, x], &grid, &mut seen, &mut current_area, &mut edges);
            let ed = handle_edges(&grid, edges, grid[y][x]);
            let ed = how_many_a(&ed, &grid, grid[y][x]);
            result += current_area.len() * ed;
            // println!("{:?}", result);
            // println!("{:?} len{:?}", ed, current_area.len());
            // println!("{ed} {}", grid[y][x]);
        }
    }
    result
}

type Grid = Vec<Vec<char>>;
type Cordinate = [usize; 2];

fn parse(input: &str) -> Grid {
    let mut grid: Grid = input
        .lines()
        .map(|line| {
            let mut line: Vec<char> = line.chars().collect();
            line.insert(0, '.');
            line.push('.');
            line
        })
        .collect();
    grid.push(vec!['.'; grid[0].len()]);
    grid.insert(0, vec!['.'; grid[0].len()]);
    grid
}

fn is_valid_index(index: Cordinate, grid: &Grid, y: i32, x: i32) -> bool {
    if index[0] as i32 + y < 0 || index[1] as i32 + x < 0 {
        return false;
    };
    if index[0] as i32 + y >= grid.len() as i32 || index[1] as i32 + x >= grid[0].len() as i32 {
        return false;
    };
    true
}

fn handle_edges(grid: &Grid, edges: HashSet<Cordinate>, target: char) -> Vec<Cordinate> {
    if edges.is_empty() {
        return vec![];
    }
    let first = edges.iter().min_by_key(|[a, b]| a + b).unwrap();
    let mut current = *first;
    let mut seen = HashSet::new();
    let mut list = vec![];
    loop {
        seen.insert(current);
        list.push(current);
        if let Some(next_cordinate) = next(&current, grid, &edges, &seen) {
            current = next_cordinate;
        } else {
            break;
        };
    }
    let mut i = 0;
    // println!("list {:?}", list);
    loop {
        if i >= list.len() - 1 {
            break;
        }
        while i < list.len() - 1 {
            match (
                list[i][0] as i32 - list[i + 1][0] as i32,
                list[i][1] as i32 - list[i + 1][1] as i32,
            ) {
                (1, 0) => {
                    list.remove(i);
                }
                (0, 1) => {
                    list.remove(i);
                }
                (0, -1) => {
                    list.remove(i);
                }
                (-1, 0) => {
                    list.remove(i);
                }
                _ => (),
            }
            i += 1;
        }
    }
    // println!("list {:?}", list);
    list
}

fn how_many_a(edges: &Vec<Cordinate>, grid: &Grid, target: char) -> usize {
    let mut result = 0;
    let directions = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    for edge in edges {
        for direction in directions.iter() {
            if is_valid_index(*edge, grid, direction[0], direction[1])
                && grid[(edge[0] as i32 + direction[0]) as usize]
                    [(edge[1] as i32 + direction[1]) as usize]
                    == target
            {
                // println!(
                //     "{:?}",
                //     [
                //         (edge[0] as i32 + direction[0]),
                //         (edge[1] as i32 + direction[1])
                //     ]
                // );
                result += 1;
            }
        }
    }
    result
}

fn next(
    cordinate: &Cordinate,
    grid: &Grid,
    edges: &HashSet<Cordinate>,
    seen: &HashSet<Cordinate>,
) -> Option<Cordinate> {
    let directions = vec![
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
        [1, 1],
        [-1, 1],
        [1, -1],
        [-1, -1],
    ];
    for direction in directions {
        if is_valid_index(*cordinate, grid, direction[0], direction[1])
            && edges.contains(&[
                (cordinate[0] as i32 + direction[0]) as usize,
                (cordinate[1] as i32 + direction[1]) as usize,
            ])
            && !seen.contains(&[
                (cordinate[0] as i32 + direction[0]) as usize,
                (cordinate[1] as i32 + direction[1]) as usize,
            ])
        {
            return Some([
                (cordinate[0] as i32 + direction[0]) as usize,
                (cordinate[1] as i32 + direction[1]) as usize,
            ]);
        }
    }
    None
}

fn traverse(
    current_cordinate: Cordinate,
    grid: &Grid,
    seen: &mut HashMap<Cordinate, usize>,
    current_area: &mut HashMap<Cordinate, usize>,
    edges: &mut HashSet<Cordinate>,
) {
    if seen.contains_key(&current_cordinate)
        || grid[current_cordinate[0]][current_cordinate[1]] == '.'
    {
        return;
    }
    let mut sides = 4;
    let current_value = grid[current_cordinate[0]][current_cordinate[1]];
    let directions = vec![[1, 0], [-1, 0], [0, 1], [0, -1]];
    seen.insert(current_cordinate, sides);
    current_area.insert(current_cordinate, sides);
    for direction in directions {
        let y = direction[0];
        let x = direction[1];
        if is_valid_index(current_cordinate, grid, y, x) {
            let new_y = (current_cordinate[0] as i32 + y) as usize;
            let new_x = (current_cordinate[1] as i32 + x) as usize;
            let next = grid[new_y][new_x];
            if next == current_value {
                traverse([new_y, new_x], grid, seen, current_area, edges);
                sides -= 1
            } else {
                edges.insert([new_y, new_x]);
            }
        }
    }
    current_area.insert(current_cordinate, sides);
    seen.insert(current_cordinate, sides);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        let result = solve(input);
        assert_eq!(result, 80);
    }
}
