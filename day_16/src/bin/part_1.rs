use std::collections::{HashMap, HashSet};

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

fn find_reinder(grid: &Grid) -> Cordinate {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if index_grid(&[y, x], grid) == 'S' {
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
    let mut not_visited: HashSet<Cordinate> = tiles.clone().into_keys().collect();
    let mut cordinate_direction: HashMap<[usize; 2], [i32; 2]> = HashMap::new();
    cordinate_direction.insert(current, [0, 1]);
    loop {
        if index_grid(&current, grid) == 'E' {
            return *tiles.get(&current).unwrap();
        }
        let current_direction = cordinate_direction.get(&current).copied().unwrap();
        let current_value = tiles.get(&current).copied().unwrap();
        not_visited.remove(&current);
        let directions: Vec<[i32; 2]> = vec![[1, 0], [0, 1], [-1, 0], [0, -1]];
        for direction in directions {
            let new_cordinate = update_cordinate(&current, &direction);
            if not_visited.contains(&new_cordinate) {
                if direction[0].abs() != current_direction[0].abs() {
                    *tiles.get_mut(&new_cordinate).unwrap() = current_value + 1001;
                } else {
                    *tiles.get_mut(&new_cordinate).unwrap() = current_value + 1;
                }
                cordinate_direction.insert(new_cordinate, direction);
            }
        }
        let (new_cordinate, _) = not_visited
            .iter()
            .filter(|key| tiles.contains_key(*key))
            .map(|key| (key, tiles.get(key).unwrap()))
            .min_by_key(|(_, value)| *value)
            .unwrap();
        current = *new_cordinate;
    }
}

fn solve(input: &str) -> i32 {
    let mut grid = parse(input);
    let mut reindear_cordinate = find_reinder(&grid);
    let mut tiles = find_tiles(&grid);
    djikstra(&mut tiles, reindear_cordinate, &grid)
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
        assert_eq!(result, 7036);
    }
}
