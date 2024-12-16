// use std::cell::RefCell;
// use std::cmp::Reverse;
// use std::collections::{BinaryHeap, HashMap, HashSet};
// use std::rc::Rc;
// use std::sync::{Arc, Mutex};
// use std::thread;

// type Branches = Vec<Arc<Mutex<Branch>>>;

// struct Branch {
//     value: Cordinate,
//     branches: Branches,
// }

// impl Branch {
//     fn new(cordinate: Cordinate) -> Self {
//         Self {
//             value: cordinate,
//             branches: vec![],
//         }
//     }

//     fn add_branch(&mut self, branch: Cordinate, branches: &HashMap<Cordinate, Arc<Mutex<Branch>>>) {
//         if let Some(next_branch) = branches.get(&branch) {
//             self.branches.push(Arc::clone(next_branch));
//         };
//     }

//     fn trim_branches(&mut self, grid: &Grid) {
//         let mut threads = vec![];
//         for i in 0..self.branches.len() {
//             let branch = Arc::clone(&self.branches[i]);
//             let grid_clone = grid.clone();
//             let thread = thread::spawn(move || {
//                 let mut branch = branch.lock().unwrap();
//                 if grid_clone[branch.value[0]][branch.value[1]] == 'E' {
//                     return Some(branch.value);
//                 }
//                 branch.trim_branches(&grid_clone);
//                 if branch.branches.is_empty() {
//                     return None;
//                 }
//                 return Some(branch.value);
//             });
//             threads.push(thread);
//         }
//         let keep: Vec<Cordinate> = threads
//             .into_iter()
//             .filter_map(|thread| thread.join().unwrap())
//             .collect();
//         self.branches.retain(|branch| {
//             keep.iter()
//                 .any(|branch_name| branch.lock().unwrap().value == *branch_name)
//         });
//     }

//     fn result(&self, grid: &mut Grid) -> i32 {
//         let mut result = self.branches.len() as i32;
//         for i in 0..self.branches.len() {
//             let branch = Arc::clone(&self.branches[i]);
//             let branch = branch.lock().unwrap();
//             grid[branch.value[0]][branch.value[1]] = 'O';
//             result += branch.result(grid);
//         }
//         result
//     }
// }

// fn main() {
//     let input = include_str!("./input.txt");
//     let result = solve(input);
//     println!("{result}");
// }

// type Grid = Vec<Vec<char>>;
// type Cordinate = [usize; 2];
// type Direction = [i32; 2];

// fn parse(input: &str) -> Grid {
//     input.lines().map(|line| line.chars().collect()).collect()
// }

// fn find_reinder(grid: &Grid) -> Cordinate {
//     for y in 0..grid.len() {
//         for x in 0..grid[0].len() {
//             if index_grid(&[y, x], grid) == 'S' {
//                 return [y, x];
//             }
//         }
//     }
//     panic!("did not find reindeer")
// }

// fn find_tiles(grid: &Grid) -> HashMap<Cordinate, i32> {
//     let mut map = HashMap::new();
//     for y in 0..grid.len() {
//         for x in 0..grid[0].len() {
//             if index_grid(&[y, x], grid) == 'S' {
//                 map.insert([y, x], 0);
//             } else if index_grid(&[y, x], grid) != '#' {
//                 map.insert([y, x], i32::MAX);
//             }
//         }
//     }
//     map
// }

// fn index_grid(cordinate: &Cordinate, grid: &Grid) -> char {
//     grid[cordinate[0]][cordinate[1]]
// }

// fn update_cordinate(current_cordinate: &Cordinate, direction: &Direction) -> Cordinate {
//     [
//         (current_cordinate[0] as i32 + direction[0]) as usize,
//         (current_cordinate[1] as i32 + direction[1]) as usize,
//     ]
// }
// fn djikstra(
//     tiles: &mut HashMap<Cordinate, i32>,
//     starting_cordinate: Cordinate,
//     grid: &Grid,
// ) -> i32 {
//     let mut current = starting_cordinate;
//     let mut heap = BinaryHeap::new();
//     let mut visited = HashSet::new();
//     let mut cordinate_direction: HashMap<[usize; 2], [i32; 2]> = HashMap::new();
//     let branches = generate_branches(tiles);
//     cordinate_direction.insert(current, [0, 1]);
//     heap.push(Reverse((0, starting_cordinate)));
//     while let Some(Reverse((distance, current))) = heap.pop() {
//         if index_grid(&current, grid) == 'E' {
//             let mut first = branches.get(&starting_cordinate).unwrap().lock().unwrap();
//             first.trim_branches(grid);
//             let mut grid = grid.clone();
//             first.result(&mut grid);
//             for line in grid.iter() {
//                 println!("{:?}", (*line).iter().collect::<String>());
//             }
//             return distance;
//         }
//         if visited.contains(&current) {
//             continue;
//         }
//         let mut branch = branches.get(&current).unwrap().lock().unwrap();
//         visited.insert(current);
//         let current_direction = cordinate_direction.get(&current.clone()).copied().unwrap();
//         let current_value = tiles.get(&current.clone()).copied().unwrap();
//         let directions: Vec<[i32; 2]> = vec![[1, 0], [0, 1], [-1, 0], [0, -1]];
//         for direction in directions {
//             let new_cordinate = update_cordinate(&current, &direction);
//             if !visited.contains(&new_cordinate) && tiles.contains_key(&new_cordinate) {
//                 if direction[0].abs() != current_direction[0].abs() {
//                     *tiles.get_mut(&new_cordinate).unwrap() = current_value + 1001;
//                     heap.push(Reverse((current_value + 1001, new_cordinate)));
//                 } else {
//                     *tiles.get_mut(&new_cordinate).unwrap() = current_value + 1;
//                     heap.push(Reverse((current_value + 1, new_cordinate)));
//                 }
//                 branch.add_branch(new_cordinate, &branches);
//                 cordinate_direction.insert(new_cordinate, direction);
//             }
//         }
//     }
//     panic!("didn't find e")
// }

// fn solve(input: &str) -> i32 {
//     let mut grid = parse(input);
//     let mut reindear_cordinate = find_reinder(&grid);
//     let mut tiles = find_tiles(&grid);
//     djikstra(&mut tiles, reindear_cordinate, &grid)
// }

// fn generate_branches(map: &HashMap<Cordinate, i32>) -> HashMap<Cordinate, Arc<Mutex<Branch>>> {
//     let mut result = HashMap::new();
//     for key in map.keys() {
//         let branch = Arc::new(Mutex::new(Branch::new(*key)));
//         result.insert(*key, branch);
//     }
//     result
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     const INPUT: &str = "###############
// #.......#....E#
// #.#.###.#.###.#
// #.....#.#...#.#
// #.###.#####.#.#
// #.#.#.......#.#
// #.#.#####.###.#
// #...........#.#
// ###.#.#####.#.#
// #...#.....#.#.#
// #.#.#.###.#.#.#
// #.....#...#.#.#
// #.###.#.#.#.#.#
// #S..#.....#...#
// ###############";
//     #[test]
//     fn failed() {
//         let result = solve(INPUT);
//         assert_eq!(result, 7046);
//     }
// }
