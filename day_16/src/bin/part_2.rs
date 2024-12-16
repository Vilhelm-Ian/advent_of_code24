use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{result}");
}

type Grid = Vec<Vec<char>>;
type Cordinate = [usize; 2];
type Direction = [i32; 2];

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_target(grid: &Grid, target: char) -> Cordinate {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if index_grid(&[y, x], grid) == target {
                return [y, x];
            }
        }
    }
    panic!("did not find reindeer")
}

fn find_tiles(grid: &Grid) -> HashMap<Cordinate, i32> {
    let mut map = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if index_grid(&[y, x], grid) == 'S' {
                map.insert([y, x], 0);
            } else if index_grid(&[y, x], grid) != '#' {
                map.insert([y, x], i32::MAX);
            }
        }
    }
    map
}

fn index_grid(cordinate: &Cordinate, grid: &Grid) -> char {
    grid[cordinate[0]][cordinate[1]]
}

fn update_cordinate(current_cordinate: &Cordinate, direction: &Direction) -> Cordinate {
    [
        (current_cordinate[0] as i32 + direction[0]) as usize,
        (current_cordinate[1] as i32 + direction[1]) as usize,
    ]
}

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

fn step(
    score: &mut i32,
    cordinate: &mut Cordinate,
    direction: &Direction,
    grid: &Grid,
) -> Option<()> {
    let new_cordinate = update_cordinate(cordinate, direction);
    let next_square = index_grid(cordinate, grid);
    if next_square == '#' {
        return None;
    }
    *score += 1;
    *cordinate = new_cordinate;
    Some(())
}

fn turn(direction: &mut Direction, score: &mut i32, i: i32) {
    let new_direciton = match (*direction, i) {
        ([0, 1], 1) | ([0, -1], 1) => [1, 0],
        ([0, 1], -1) | ([0, -1], -1) => [-1, 0],
        ([1, 0], 1) | ([-1, 0], 1) => [0, 1],
        ([-1, 0], -1) | ([1, 0], -1) => [0, -1],
        _ => panic!("not valid  direction or i is not unit"),
    };
    *direction = new_direciton;
    *score += 1000;
}

fn djikstra(
    tiles: &mut HashMap<Cordinate, i32>,
    starting_cordinate: Cordinate,
    grid: &Grid,
) -> i32 {
    let mut current = starting_cordinate;
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut cordinate_direction: HashMap<[usize; 2], [i32; 2]> = HashMap::new();
    cordinate_direction.insert(current, [0, 1]);
    heap.push(Reverse((0, starting_cordinate)));
    while let Some(Reverse((distance, current))) = heap.pop() {
        if index_grid(&current, grid) == 'E' {
            return distance;
        }
        visited.insert(current);
        let current_direction = cordinate_direction.get(&current.clone()).copied().unwrap();
        let current_value = tiles.get(&current.clone()).copied().unwrap();
        let directions: Vec<[i32; 2]> = vec![[1, 0], [0, 1], [-1, 0], [0, -1]];
        let mut tmp = current_value;
        for direction in directions {
            let new_cordinate = update_cordinate(&current, &direction);
            if !visited.contains(&new_cordinate) && tiles.contains_key(&new_cordinate) {
                if direction[0].abs() != current_direction[0].abs() {
                    let mut temp = tiles.get_mut(&new_cordinate).unwrap();
                    if current_value + 1001 < *temp {
                        *temp = current_value + 1001;
                        heap.push(Reverse((current_value + 1001, new_cordinate)));
                    };
                } else {
                    let mut temp = tiles.get_mut(&new_cordinate).unwrap();
                    if current_value + 1 < *temp {
                        *temp = current_value + 1;
                        heap.push(Reverse((current_value + 1, new_cordinate)));
                    };
                }
                cordinate_direction.insert(new_cordinate, direction);
            }
        }
    }
    0
}

fn solve(input: &str) -> i32 {
    let mut grid = parse(input);
    let mut reindear_cordinate = find_target(&grid, 'S');
    let mut tiles = find_tiles(&grid);
    let mut e = find_target(&grid, 'E');
    djikstra(&mut tiles, reindear_cordinate, &grid);
    let mut visited = HashSet::new();
    let result = first_search(&tiles, e, &mut visited, &grid);
    let mut temp = vec![vec![1111; grid[0].len()]; grid.len()];
    for (cordinate, value) in tiles.iter() {
        temp[cordinate[0]][cordinate[1]] = *value;
    }
    for line in temp {
        println!("{:?}", line);
    }
    for square in visited.iter() {
        grid[square[0]][square[1]] = 'O';
    }
    for line in grid {
        println!("{:?}", line.iter().collect::<String>());
    }
    visited.len() as i32 + 1
}

fn first_search(
    tiles: &HashMap<Cordinate, i32>,
    cordinate: Cordinate,
    visited: &mut HashSet<Cordinate>,
    grid: &Grid,
) -> bool {
    let mut result = false;
    let mut directions = vec![[0, 1], [0, -1], [1, 0], [-1, 0]];
    let current_value = tiles.get(&cordinate).unwrap();
    if index_grid(&cordinate, grid) == 'S' {
        return true;
    }
    for direction in directions {
        let new_cordinate = update_cordinate(&cordinate, &direction);
        let mut direction = direction;
        direction[0] *= 2;
        direction[1] *= 2;
        let new_cordinate_2 = update_cordinate(&cordinate, &direction);
        if let Some(new_value) = tiles.get(&new_cordinate) {
            if *new_value == *current_value - 1 {
                result = first_search(&tiles, new_cordinate, visited, grid);
            } else if *new_value == *current_value - 1001 {
                result = first_search(&tiles, new_cordinate, visited, grid);
                if let Some(new_value) = tiles.get(&new_cordinate_2) {
                    if *new_value == *current_value - 2 {
                        result = first_search(&tiles, new_cordinate_2, visited, grid);
                    }
                }
            }
        }
    }
    if result {
        visited.insert(cordinate);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    #[test]
    fn it_works() {
        let result = solve(INPUT);
        assert_eq!(result, 45);
    }
    const BIG_INPUT: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
    #[test]
    fn it_works_2() {
        let result = solve(BIG_INPUT);
        assert_eq!(result, 64);
    }
}
