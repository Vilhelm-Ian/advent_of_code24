use std::{
    collections::{HashMap, HashSet},
    primitive,
};

pub const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

fn main() {
    traverse();
}

type Grid = Vec<Vec<char>>;

type Cordinate = [usize; 2];

fn is_valid_index(index: Cordinate, grid: &Grid, y: i32, x: i32) -> bool {
    if index[0] as i32 + y < 0 || index[1] as i32 + x < 0 {
        return false;
    };
    if index[0] as i32 + y >= grid.len() as i32 || index[1] as i32 + x >= grid[0].len() as i32 {
        return false;
    };
    true
}

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn traverse() {
    let mut grid = parse(INPUT);
    let y = 0;
    let x = 4;
    grid[y][x] = 'O';
    for m in 1..=6 {
        let mut grid_clone = grid.clone();
        name_later(&mut grid_clone, y, x, m, '#');
        name_later(&mut grid_clone, y, x, m * 2, 'X');
        print!("\x1B[2J\x1B[H");
        std::thread::sleep(std::time::Duration::from_secs(2));
        for i in 0..grid_clone.len() {
            println!("{:?} {:?}", grid_clone[i], grid[i]);
        }
        println!("\n");
    }
}

fn name_later(grid: &mut Grid, y: usize, x: usize, limit: usize, ch: char) {
    for i in 0..=limit {
        if is_valid_index([y, 4], grid, i as i32, limit as i32 - i as i32) {
            grid[y + i][x + limit - i] = ch;
        }
        if is_valid_index([y, x], grid, i as i32, i as i32 - limit as i32) {
            grid[y + i][(x as i32 - limit as i32 + i as i32) as usize] = ch;
        }
        if is_valid_index([y, x], grid, -(i as i32), limit as i32 - i as i32) {
            grid[y - i][x + limit - i] = ch;
        }
        if is_valid_index([y, x], grid, -(i as i32), -(limit as i32) + i as i32) {
            grid[y - i][(x as i32 - limit as i32 + i as i32) as usize] = ch;
        }
    }
}
