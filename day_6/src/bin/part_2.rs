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
    let mut guard_position = find_starting_positions(&grid);
    let mut result = HashSet::new();
    while let Some(new_position) = move_guard(&guard_position, &mut grid) {
        guard_position = new_position;
        detect_loop(&guard_position, &grid, &mut result);
    }
    result.len()
}

type Cordinate = [usize; 2];

fn find_starting_positions(grid: &Grid) -> Cordinate {
    for (y, row) in grid.iter().enumerate() {
        for x in 0..row.len() {
            match grid[y][x] {
                'v' | '^' | '>' | '<' => return [y, x],
                _ => (),
            };
        }
    }
    panic!("couldn't find sarting")
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

fn move_guard(guard_position: &Cordinate, grid: &mut Grid) -> Option<Cordinate> {
    let mut new_position = *guard_position;
    match grid[guard_position[0]][guard_position[1]] {
        '^' => update(&mut new_position, grid, -1, 0)?,
        'v' => update(&mut new_position, grid, 1, 0)?,
        '>' => update(&mut new_position, grid, 0, 1)?,
        '<' => update(&mut new_position, grid, 0, -1)?,
        _ => panic!("not guard"),
    };
    Some(new_position)
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
    let old_y = guard_position[0];
    let old_x = guard_position[1];
    let new_y = (guard_position[0] as i32 + y) as usize;
    let new_x = (guard_position[1] as i32 + x) as usize;
    if grid[new_y][new_x] == '#' {
        grid[guard_position[0]][guard_position[1]] =
            turn(grid[guard_position[0]][guard_position[1]]);
        return Some(());
    }
    guard_position[0] = new_y;
    guard_position[1] = new_x;
    grid[new_y][new_x] = grid[old_y][old_x];
    grid[old_y][old_x] = 'X';
    Some(())
}

fn detect_loop(guard_position: &Cordinate, grid: &Grid, result: &mut HashSet<Cordinate>) -> bool {
    let mut guard_position_1 = *guard_position;
    let mut guard_position_2 = *guard_position;
    if let Some(wall_cordinate) = add_wall(guard_position, grid) {
        let mut new_grid = grid.clone();
        new_grid[wall_cordinate[0]][wall_cordinate[1]] = '#';
        let mut new_grid_2 = new_grid.clone();
        let mut i = 0;
        // harris and tortioise algorithm
        while let Some(new_position_1) = move_guard(&guard_position_1, &mut new_grid) {
            i += 1;
            guard_position_1 = new_position_1;
            if i % 2 == 0 {
                if let Some(new_position_2) = move_guard(&guard_position_2, &mut new_grid_2) {
                    guard_position_2 = new_position_2;
                    if new_grid_2[new_position_2[0]][new_position_2[1]]
                        == new_grid[new_position_1[0]][new_position_1[1]]
                        && new_position_2.eq(&new_position_1)
                    {
                        result.insert(wall_cordinate);
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn add_wall(guard_position: &Cordinate, grid: &Grid) -> Option<Cordinate> {
    let (y, x) = match grid[guard_position[0]][guard_position[1]] {
        '^' => (-1, 0),
        'v' => (1, 0),
        '>' => (0, 1),
        '<' => (0, -1),
        _ => panic!("not guard"),
    };
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
    Some([new_y, new_x])
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    #[test]
    fn it_works() {
        let result = solve(INPUT);
        assert_eq!(result, 6);
    }
}
