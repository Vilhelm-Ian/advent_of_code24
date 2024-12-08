use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{result}");
}

type Grid = Vec<Vec<char>>;

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

type Cordinate = [usize; 2];
type AntenaMap = HashMap<char, Vec<Cordinate>>;

fn solve(input: &str) -> usize {
    let grid = parse(input);
    let mut result = HashSet::new();
    let mut map: AntenaMap = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            add_index(y, x, &grid, &mut map);
        }
    }
    for antenas in map.values() {
        for antena in antenas {
            result.insert(*antena);
        }
    }
    for (_, cordinates) in map {
        for i in 0..cordinates.len() {
            for z in 0..cordinates.len() {
                if i == z {
                    continue;
                }
                if let Some(cordinate) = are_in_line(cordinates[i], cordinates[z], &grid) {
                    result.insert(cordinate);
                }
            }
        }
    }
    result.len()
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

fn add_index(y: usize, x: usize, grid: &Grid, map: &mut AntenaMap) {
    if grid[y][x] != '.' {
        (*map.entry(grid[y][x]).or_default()).push([y, x]);
    }
}

fn are_in_line(cor_1: Cordinate, cor_2: Cordinate, grid: &Grid) -> Option<Cordinate> {
    let new_y = cor_1[0] as i32 - cor_2[0] as i32;
    let new_x = cor_1[1] as i32 - cor_2[1] as i32;
    if is_valid_index([cor_1[0], cor_1[1]], grid, new_y, new_x) {
        return Some([
            (cor_1[0] as i32 + new_y) as usize,
            (cor_1[1] as i32 + new_x) as usize,
        ]);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
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
    #[test]
    fn it_works() {
        let result = solve(INPUT);
        assert_eq!(result, 14);
    }

    #[test]
    fn simple() {
        let result = solve(
            "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........",
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn three_a() {
        let result = solve(
            "..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........",
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn four_a() {
        let result = solve(
            "..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........",
        );
        assert_eq!(result, 3);
    }
}
