use std::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input, 71, 1024);
    println!("{result}");
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

fn solve(input: &str, grid_dimensions: usize, falling: usize) -> i32 {
    let bytes = parse(input);
    let mut grid = vec![vec!['.'; grid_dimensions]; grid_dimensions];
    for i in 0..falling {
        grid[bytes[i][0]][bytes[i][1]] = '#';
    }
    let mut tiles = generate_tiles(&grid);
    djikstra(&mut tiles, [0, 0], &grid)
}

fn djikstra(
    tiles: &mut HashMap<Cordinate, i32>,
    starting_cordinate: Cordinate,
    grid: &Grid,
) -> i32 {
    let mut current = starting_cordinate;
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(Reverse((0, starting_cordinate)));
    let mut grid = grid.clone();
    while let Some(Reverse((distance, current))) = heap.pop() {
        if current == [grid.len() - 1, grid[0].len() - 1] {
            return distance;
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
    panic!("didn't find e")
}

fn generate_tiles(grid: &Grid) -> HashMap<Cordinate, i32> {
    let mut result = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '#' {
                if y == 0 && x == 0 {
                    result.insert([y, x], 0);
                    continue;
                }
                result.insert([y, x], i32::MAX);
            }
        }
    }
    result
}

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
        assert_eq!(result, 22);
    }
}
