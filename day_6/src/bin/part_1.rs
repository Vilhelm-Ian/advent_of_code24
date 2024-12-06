use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

type Grid = Vec<Vec<char>>;

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve(input: &str) -> usize {
    let mut grid = parse(input);
    let (mut guard_position, crates_position) = find_starting_positions(&grid);
    let mut visited: HashMap<Cordinate, Vec<char>> = HashMap::new();
    loop {
        if move_guard(&mut guard_position, &mut grid).is_none() {
            break;
        };
        (*visited.entry(guard_position).or_default())
            .push(grid[guard_position[0]][guard_position[1]]);
    }

    visited.len()
}

type Cordinate = [usize; 2];

fn find_starting_positions(grid: &Grid) -> (Cordinate, HashSet<Cordinate>) {
    let mut crates_position = HashSet::new();
    let mut guard_position = [0, 0];
    for (y, row) in grid.iter().enumerate() {
        for x in 0..row.len() {
            match grid[y][x] {
                '#' => {
                    crates_position.insert([y, x]);
                }
                'v' | '^' | '>' | '<' => {
                    crates_position.insert([y, x]);
                }
                _ => (),
            };
        }
    }
    (guard_position, crates_position)
}

fn turn(guard: char) -> char {
    match guard {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("not guard"),
    }
}

fn move_guard(guard_position: &mut Cordinate, grid: &mut Grid) -> Option<()> {
    match grid[guard_position[0]][guard_position[1]] {
        '^' => update(guard_position, grid, -1, 0)?,
        'v' => update(guard_position, grid, 1, 0)?,
        '>' => update(guard_position, grid, 0, 1)?,
        '<' => update(guard_position, grid, 0, -1)?,
        _ => panic!("not guard"),
    };
    Some(())
}

fn update(guard_position: &mut Cordinate, grid: &mut Grid, y: i32, x: i32) -> Option<()> {
    if guard_position[0] == 0 && y == -1 {
        return None;
    }
    if guard_position[0] == grid.len() - 1 && y == 1 {
        return None;
    }
    if guard_position[1] == 0 && x == -1 {
        return None;
    }
    if guard_position[1] == grid[0].len() - 1 && x == 1 {
        return None;
    }
    let new_y = (guard_position[0] as i32 + y) as usize;
    let new_x = (guard_position[1] as i32 + x) as usize;
    if grid[new_y][new_x] != '.' {
        grid[guard_position[0]][guard_position[1]] =
            turn(grid[guard_position[0]][guard_position[1]]);
        return Some(());
    }
    guard_position[0] = new_y;
    guard_position[1] = new_x;
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const INPUT: &str = "";
    #[test]
    fn it_works() {
        let result = INPUT;
        assert_eq!(result, "hello");
    }
}
