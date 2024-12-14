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
    let mut result = 0;
    let target = 'A';
    let mut result = 0;
    println!("{:?}", grid);
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == target {
                continue;
            }
            if (y < grid.len() - 1 && grid[y + 1][x] == target)
                && ((x > 0 && grid[y][x - 1] == target)
                    || (x < grid.len() - 1 && grid[y][x + 1] == target))
            {
                result += 1;
            }
            if (y > 0 && grid[y - 1][x] == target)
                && ((x > 0 && grid[y][x - 1] == target)
                    || (x < grid.len() - 1 && grid[y][x + 1] == target))
            {
                result += 1;
            }
        }
    }
    println!("{:?}", result);
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

    #[test]
    fn test_2() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let result = solve(input);
        assert_eq!(result, 80);
    }
    #[test]
    fn test_3() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        let result = solve(input);
        assert_eq!(result, 80);
    }
}
