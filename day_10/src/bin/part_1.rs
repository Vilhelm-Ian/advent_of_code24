use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

fn solve(input: &str) -> i32 {
    let grid = parse(input);
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                result += traverse(grid[y][x], [y, x], &grid, &mut HashSet::new());
            }
        }
    }
    result
}

fn traverse(
    current_value: i32,
    current_cordinate: Cordinate,
    grid: &Grid,
    mut set: &mut HashSet<Cordinate>,
) -> i32 {
    if current_value == 9 {
        set.insert(current_cordinate);
    }
    if is_valid_index(current_cordinate, grid, 1, 0) {
        let mut new_cordinate = current_cordinate;
        new_cordinate[0] += 1;
        let next = grid[new_cordinate[0]][new_cordinate[1]];
        if next - current_value == 1 {
            traverse(next, new_cordinate, grid, &mut set);
        }
    }
    if is_valid_index(current_cordinate, grid, -1, 0) {
        let mut new_cordinate = current_cordinate;
        new_cordinate[0] -= 1;
        let next = grid[new_cordinate[0]][new_cordinate[1]];
        if next - current_value == 1 {
            traverse(next, new_cordinate, grid, &mut set);
        }
    }
    if is_valid_index(current_cordinate, grid, 0, 1) {
        let mut new_cordinate = current_cordinate;
        new_cordinate[1] += 1;
        let next = grid[new_cordinate[0]][new_cordinate[1]];
        if next - current_value == 1 {
            traverse(next, new_cordinate, grid, &mut set);
        }
    }
    if is_valid_index(current_cordinate, grid, 0, -1) {
        let mut new_cordinate = current_cordinate;
        new_cordinate[1] -= 1;
        let next = grid[new_cordinate[0]][new_cordinate[1]];
        if next - current_value == 1 {
            traverse(next, new_cordinate, grid, &mut set);
        }
    }
    set.len() as i32
}

type Grid = Vec<Vec<i32>>;

fn parse(input: &str) -> Grid {
    let input = input.trim();
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

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

#[cfg(test)]
mod tests {
    use super::*;
    //     #[test]
    //     fn first_1() {
    //         let input = "...0...
    // ...1...
    // ...2...
    // 6543456
    // 7.....7
    // 8.....8
    // 9.....9";
    //         let grid = parse(input);
    //         let result = traverse(0, [0, 3], &grid);
    //         assert_eq!(result, 2);
    //     }
    //     #[test]
    //     fn first_2() {
    //         let input = "..90..9
    // ...1.98
    // ...2..7
    // 6543456
    // 765.987
    // 876....
    // 987....";
    //         let grid = parse(input);
    //         let result = traverse(0, [0, 3], &grid);
    //         assert_eq!(result, 4);
    //     }
    //     #[test]
    //     fn first_3() {
    //         let input = "10..9..
    // 2...8..
    // 3...7..
    // 4567654
    // ...8..3
    // ...9..2
    // .....01";
    //         let result = solve(input);
    //         assert_eq!(result, 3);
    //     }
    #[test]
    fn first_4() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let result = solve(input);
        assert_eq!(result, 36);
    }
}
