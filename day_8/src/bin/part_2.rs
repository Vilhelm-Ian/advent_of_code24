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
    let mut grid = parse(input);
    let mut result = HashSet::new();
    let mut map: AntenaMap = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            add_index(y, x, &grid, &mut map);
        }
    }
    for (_, cordinates) in map {
        for i in 0..cordinates.len() {
            for z in 0..cordinates.len() {
                if i == z {
                    continue;
                }
                result.insert(cordinates[i]);
                result.insert(cordinates[z]);
                if let Some(difference) = get_difference(cordinates[i], cordinates[z], &grid) {
                    iterate_cordinates(cordinates[i], &grid, &mut result, difference);
                }
            }
        }
    }
    for cordinate in result.iter() {
        if grid[cordinate[0]][cordinate[1]] == '.' {
            grid[cordinate[0]][cordinate[1]] = '#';
        }
    }
    for line in grid {
        println!("{:?}", line);
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

fn get_difference(cor_1: Cordinate, cor_2: Cordinate, grid: &Grid) -> Option<([i32; 2])> {
    let new_y = cor_1[0] as i32 - cor_2[0] as i32;
    let new_x = cor_1[1] as i32 - cor_2[1] as i32;
    if is_valid_index([cor_1[0], cor_1[1]], grid, new_y, new_x) {
        return Some([new_y, new_x]);
    }
    None
}

fn iterate_cordinates(
    cordinate: Cordinate,
    grid: &Grid,
    set: &mut HashSet<Cordinate>,
    difference: [i32; 2],
) {
    let mut current_cordinate = cordinate;
    while is_valid_index(current_cordinate, grid, difference[0], difference[1]) {
        current_cordinate[0] = (current_cordinate[0] as i32 + difference[0]) as usize;
        current_cordinate[1] = (current_cordinate[1] as i32 + difference[1]) as usize;
        set.insert(current_cordinate);
    }
    let mut current_cordinate = cordinate;
    while is_valid_index(current_cordinate, grid, -difference[0], -difference[1]) {
        current_cordinate[0] = (current_cordinate[0] as i32 - difference[0]) as usize;
        current_cordinate[1] = (current_cordinate[1] as i32 - difference[1]) as usize;
        set.insert(current_cordinate);
    }
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
    fn it_works_part_2() {
        let result = solve(INPUT);
        assert_eq!(result, 34);
    }

    #[test]
    fn first_example() {
        let result = solve(
            "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........",
        );
        assert_eq!(result, 9);
    }
}
