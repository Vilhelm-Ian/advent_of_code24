use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn solve(input: &str) -> i32 {
    let grid = parse(input);
    let starting = find_element('S', &grid);
    let mut cordinates = breath_first_search(&grid, &starting);
    let mut result = 0;
    for (cordinate, distance) in cordinates.iter() {
        cordinates
            .iter()
            .filter(|(cordinate_2, _)| manhatan_distance(&cordinate, &cordinate_2) <= 20)
            .for_each(|(cordinate_2, distance_2)| {
                let manhatan = manhatan_distance(&cordinate, &cordinate_2);
                if distance_2 - distance - manhatan >= 100 {
                    result += 1;
                }
            });
    }
    result
}

type Grid = Vec<Vec<char>>;
type Cordinate = [usize; 2];
type Direction = [i32; 2];

fn manhatan_distance(a: &Cordinate, b: &Cordinate) -> i32 {
    (a[0] as i32 - b[0] as i32).abs() + (a[1] as i32 - b[1] as i32).abs()
}

fn find_element(target: char, grid: &Grid) -> Cordinate {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == target {
                return [y, x];
            }
        }
    }
    panic!("didn't find element")
}

fn update_cordinate(current_cordinate: Cordinate, direction: Direction) -> Cordinate {
    [
        (current_cordinate[0] as i32 + direction[0]) as usize,
        (current_cordinate[1] as i32 + direction[1]) as usize,
    ]
}

fn index_grid(cordinate: Cordinate, grid: &Grid) -> char {
    grid[cordinate[0]][cordinate[1]]
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

fn breath_first_search(grid: &Grid, cordinate: &Cordinate) -> HashMap<Cordinate, i32> {
    let directions = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    let mut que = VecDeque::new();
    let mut visited = HashMap::new();
    let mut grid_2 = grid.clone();
    que.push_front((*cordinate, true, visited.clone(), 0));
    let mut no_cheats = i32::MAX;
    while let Some((cordinate, can_move_through_wall, mut visited_temp, steps)) = que.pop_back() {
        if index_grid(cordinate, grid) == 'E' {
            visited_temp.insert(cordinate, steps);
            visited = visited_temp;
            no_cheats = steps;
            break;
        }
        if visited_temp.contains_key(&cordinate) {
            continue;
        }
        visited_temp.insert(cordinate, steps);
        for direction in directions {
            if !is_valid_index(cordinate, grid, direction[0], direction[1]) {
                continue;
            }
            let new_cordinate = update_cordinate(cordinate, direction);
            if matches!(index_grid(new_cordinate, grid), '.' | 'E') {
                que.push_front((
                    new_cordinate,
                    can_move_through_wall,
                    visited_temp.clone(),
                    steps + 1,
                ));
            }
        }
    }
    visited
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    #[test]
    fn it_works() {
        let result = solve(INPUT);
        assert_eq!(result, 6);
    }
}
