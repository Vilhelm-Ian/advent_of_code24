use std::collections::HashMap;

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
            let mut current_area = HashMap::new();
            if seen.contains_key(&[y, x]) {
                continue;
            }
            traverse([y, x], &grid, &mut seen, &mut current_area);
            result += current_area.len() * current_area.values().sum::<usize>();
        }
    }
    result
}

type Grid = Vec<Vec<char>>;
type Cordinate = [usize; 2];

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn index_grid(grid: &Grid, y: usize, x: usize) -> Option<char> {
    if is_valid_index([y, x], grid, 0, 0) {
        return Some(grid[y][x]);
    }
    None
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
) {
    if seen.contains_key(&current_cordinate) {
        return ();
    }
    let mut sides = 4;
    let current_value = grid[current_cordinate[0]][current_cordinate[1]];
    let directions = vec![[1, 0], [-1, 0], [0, 1], [0, -1]];
    seen.insert(current_cordinate, sides);
    for direction in directions {
        let y = direction[0];
        let x = direction[1];
        if is_valid_index(current_cordinate, grid, y, x) {
            let new_y = (current_cordinate[0] as i32 + y) as usize;
            let new_x = (current_cordinate[1] as i32 + x) as usize;
            let next = grid[new_y][new_x];
            if next == current_value {
                traverse([new_y, new_x], grid, seen, current_area);
                sides -= 1
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
        assert_eq!(result, 140);
    }
}
