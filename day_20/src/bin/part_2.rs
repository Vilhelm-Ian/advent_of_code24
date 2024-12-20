use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

type Grid = Vec<Vec<char>>;
type Cordinate = [usize; 2];
type Direction = [i32; 2];

fn solve(input: &str) -> i32 {
    let grid = parse(input);
    let starting_cordinate = find_element(&grid, 'S');
    breath_first_search(&grid, &starting_cordinate, [0, 0])
}

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn update_cordinate(current_cordinate: Cordinate, direction: Direction) -> Cordinate {
    [
        (current_cordinate[0] as i32 + direction[0]) as usize,
        (current_cordinate[1] as i32 + direction[1]) as usize,
    ]
}

fn index_grid(cordinate: &Cordinate, grid: &Grid) -> char {
    grid[cordinate[0]][cordinate[1]]
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

fn find_element(grid: &Grid, element: char) -> Cordinate {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == element {
                return [y, x];
            }
        }
    }
    panic!("couldn't find {element} in grid")
}

fn find_tiles(grid: &Grid) -> HashMap<Cordinate, (i32, bool)> {
    let mut result = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x] == '.' {
                result.insert([y, x], (0, true));
            } else if grid[y][x] == '#' && count_neighbours(grid, &[y, x]) > 1 {
                result.insert([y, x], (0, false));
            }
        }
    }
    result
}

fn count_neighbours(grid: &Grid, cordinate: &Cordinate) -> usize {
    let directions = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    directions
        .iter()
        .map(|direction| update_cordinate(*cordinate, *direction))
        .filter(|cordinate| {
            is_valid_index(*cordinate, grid, [0, 0]) && index_grid(cordinate, grid) == '.'
        })
        .count()
}

fn breath_first_search(grid: &Grid, cordinate: &Cordinate, prev_direction: Direction) -> i32 {
    let directions = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    let mut que = VecDeque::new();
    let mut visited = HashMap::new();
    let mut grid_2 = grid.clone();
    que.push_front((*cordinate, true, visited.clone(), 0));
    let mut no_cheats = i32::MAX;
    while let Some((cordinate, can_move_through_wall, mut visited_temp, steps)) = que.pop_back() {
        if index_grid(&cordinate, grid) == 'E' {
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
            if !is_valid_index(cordinate, grid, direction) {
                continue;
            }
            let new_cordinate = update_cordinate(cordinate, direction);
            if matches!(index_grid(&new_cordinate, grid), '.' | 'E') {
                que.push_front((
                    new_cordinate,
                    can_move_through_wall,
                    visited_temp.clone(),
                    steps + 1,
                ));
            }
        }
    }
    let mut que = VecDeque::new();
    que.push_front((*cordinate, true, HashMap::new(), 0, (None, 20)));
    let mut result = 0;
    let mut i = 0;
    let mut blabla = HashSet::new();
    // let mut cheats = HashSet::new();
    while let Some((
        cordinate,
        can_move_through_wall,
        mut visited_temp,
        steps,
        (mut starting, cheats_allowed),
    )) = que.pop_back()
    {
        if starting.is_none()
        if steps > no_cheats {
            continue;
        }
        if index_grid(&cordinate, grid) == 'E' {
            break;
        }
        if visited_temp.contains_key(&cordinate) {
            continue;
        }
        visited_temp.insert(cordinate, steps);
        for direction in directions {
            if !is_valid_index(cordinate, grid, direction) {
                continue;
            }
            let new_cordinate = update_cordinate(cordinate, direction);
            if matches!(index_grid(&new_cordinate, grid), '.' | 'E') {
                if cheats_allowed < 20 {
                    let temp = visited.get(&new_cordinate).unwrap();
                    if temp - steps > 0 {
                        i += 1;
                        // if i > 200 {
                        //     panic!("hello")
                        // }
                        println!("{:?}", [starting.unwrap(), new_cordinate]);
                        // println!("{:?}", temp - steps);
                    }
                    continue;
                }
                que.push_front((
                    new_cordinate,
                    can_move_through_wall,
                    visited_temp.clone(),
                    steps + 1,
                    (None, cheats_allowed),
                ));
            }
            if cheats_allowed > 0 && index_grid(&new_cordinate, grid) == '#' {
                if starting.is_none() && steps < 20 {
                    continue;
                }
                if starting.is_none() {
                    starting = Some(cordinate);
                }
                que.push_front((
                    new_cordinate,
                    can_move_through_wall,
                    visited_temp.clone(),
                    steps + 1,
                    (starting, cheats_allowed - 1),
                ));
                // let temp = visited.get(&new_new_cordinate).unwrap();
                // if temp - steps - 2 >= 100 {
                //     result += 1
                // }
            }
        }
    }
    println!("{no_cheats}");
    // println!("{:?}", cheats.len());
    // for cheat in cheats {
    //     let difference = cheat
    //         .iter()
    //         .map(|cordinate| {
    //             visited
    //                 .get(cordinate)
    //                 .expect(format!("couldn't get {:?}", cordinate).as_str())
    //         })
    //         .fold(0, |acc, x| acc - x);
    //     println!("{:?} {:?}", cheat, difference);
    // }
    result
}

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
    fn test_1_2() {
        let result = solve(INPUT);
        assert_eq!(result, 1);
    }
}
