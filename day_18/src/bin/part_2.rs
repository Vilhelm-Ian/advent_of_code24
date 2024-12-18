use std::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input, 71, 1024);
    println!("{:?}", result);
}

fn parse(input: &str) -> Vec<[usize; 2]> {
    input
        .lines()
        .map(|line| {
            let mut cordinates = line.split(',').map(|num| num.parse().unwrap());
            let x = cordinates.next().unwrap();
            let y = cordinates.next().unwrap();
            [y, x]
        })
        .collect()
}

type Grid = Vec<Vec<char>>;
type Cordinate = [usize; 2];
type Direction = [i32; 2];

fn is_valid_index(index: Cordinate, grid: &Grid, direction: Direction) -> bool {
    if index[0] as i32 + direction[0] < 0 || index[1] as i32 + direction[1] < 0 {
        return false;
    };
    if index[0] as i32 + direction[0] >= grid.len() as i32
        || index[1] as i32 + direction[1] >= grid[0].len() as i32
    {
        return false;
    };
    true
}

fn index_grid(cordinate: &Cordinate, grid: &Grid) -> char {
    grid[cordinate[0]][cordinate[1]]
}

fn update_cordinate(current_cordinate: Cordinate, direction: Direction) -> Cordinate {
    [
        (current_cordinate[0] as i32 + direction[0]) as usize,
        (current_cordinate[1] as i32 + direction[1]) as usize,
    ]
}

fn solve(input: &str, grid_dimensions: usize, falling: usize) -> Cordinate {
    let mut bytes = parse(input);
    let mut grid = vec![vec!['.'; grid_dimensions]; grid_dimensions];
    for i in 0..falling {
        grid[bytes[i][0]][bytes[i][1]] = '#';
    }
    let mut tiles = generate_tiles(&grid, [0, 0]);
    bytes = bytes[falling..].to_vec();
    let result = djikstra(
        &mut tiles,
        [0, 0],
        &grid,
        [grid.len() - 1, grid[0].len() - 1],
    );
    for byte in bytes.iter() {
        grid[byte[0]][byte[1]] = '#';
    }
    for byte in bytes.iter().rev() {
        grid[byte[0]][byte[1]] = '.';
        let mut tiles = generate_tiles(&grid, [0, 0]);
        let result = djikstra(
            &mut tiles,
            [0, 0],
            &grid,
            [grid.len() - 1, grid[0].len() - 1],
        );
        if result.is_some() {
            return [byte[1], byte[0]];
        }
    }
    panic!("didn't find it")
}

fn djikstra(
    tiles: &mut HashMap<Cordinate, i32>,
    starting_cordinate: Cordinate,
    grid: &Grid,
    target: Cordinate,
) -> Option<i32> {
    let mut current = starting_cordinate;
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(Reverse((0, starting_cordinate)));
    let mut grid = grid.clone();
    while let Some(Reverse((distance, current))) = heap.pop() {
        if current == target {
            return Some(distance);
        }
        visited.insert(current);
        let current_value = tiles.get(&current.clone()).copied().unwrap();
        let directions: Vec<[i32; 2]> = vec![[1, 0], [0, 1], [-1, 0], [0, -1]];
        for direction in directions {
            let new_cordinate = update_cordinate(current, direction);
            if !visited.contains(&new_cordinate) && tiles.contains_key(&new_cordinate) {
                let tile = tiles.get_mut(&new_cordinate).unwrap();
                if *tile > current_value + 1 {
                    grid[current[0]][current[1]] =
                        char::from_digit((current_value as u32 + 1) % 10, 10).unwrap();
                    *tile = current_value + 1;
                    heap.push(Reverse((current_value + 1, new_cordinate)));
                }
            }
        }
    }
    None
}

fn generate_tiles(grid: &Grid, start: Cordinate) -> HashMap<Cordinate, i32> {
    let mut result = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '#' {
                if y == start[0] && x == start[1] {
                    result.insert([y, x], 0);
                    continue;
                }
                result.insert([y, x], i32::MAX);
            }
        }
    }
    result
}

// fn breath_first_search(
//     step: usize,
//     grid: &Grid,
//     current_cordinate: Cordinate,
//     visited: &mut HashSet<Cordinate>,
//     bytes: &Vec<Cordinate>,
//     max: &mut i32,
// ) -> i32 {
//     let directions = [[1, 0], [-1, 0], [0, 1], [0, -1]];
//     let mut grid = grid.clone();
//     grid[current_cordinate[0]][current_cordinate[1]] = 'O';
//     if step as i32 >= *max {
//         return *max;
//     }
//     if current_cordinate[0] == grid.len() - 1 && current_cordinate[1] == grid[0].len() - 1 {
//         for line in grid {
//             println!("{:?}", line);
//         }
//         *max = step as i32;
//         println!("\n");
//         // println!("{:?}", step);
//         return *max;
//     }
//     if visited.contains(&current_cordinate) {
//         return *max;
//     }
//     visited.insert(current_cordinate);
//     for direction in directions {
//         if !is_valid_index(current_cordinate, &grid, direction) {
//             continue;
//         }
//         let new_cordinaate = update_cordinate(current_cordinate, direction);
//         if index_grid(&new_cordinaate, &grid) == '#' {
//             continue;
//         }
//         breath_first_search(
//             step + 1,
//             &grid,
//             new_cordinaate,
//             &mut visited.clone(),
//             bytes,
//             max,
//         );
//     }
//     *max
// }

#[cfg(test)]
mod tests {
    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    use super::*;
    #[test]
    fn it_works() {
        let result = solve(INPUT, 7, 12);
        assert_eq!(result, [6, 1]);
    }
}
