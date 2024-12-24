use std::{
    any::Any,
    arch::x86_64::_CMP_TRUE_UQ,
    char,
    collections::{HashMap, HashSet, VecDeque},
    env::current_dir,
};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

fn solve(input: &str) -> u32 {
    let mut grid = parse(input);
    let mut result = 0;
    let mut seen = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if seen.contains_key(&[y, x]) {
                continue;
            }
            let mut current_area = HashMap::new();
            traverse([y, x], &grid, &mut seen, &mut current_area);
            let mut edges: HashMap<[usize; 2], usize> = current_area.clone();
            edges.retain(|_, value| *value > 0);
            let perimeter = find_corners([y, x], edges, &grid, current_area.clone());
            let price = perimeter * current_area.len() as u32;
            println!("{:?}", perimeter);
            println!("{:?}", current_area);
            result += price;
        }
    }
    // for line in grid {
    //     println! {"g{:?}",line};
    // }
    result
}

type Grid = Vec<Vec<char>>;
type Cordinate = [usize; 2];

fn parse(input: &str) -> Grid {
    let mut grid: Grid = input
        .lines()
        .map(|line| {
            let mut line: Vec<char> = line.chars().collect();
            line.insert(0, '.');
            line.push('.');
            line
        })
        .collect();
    grid.push(vec!['.'; grid[0].len()]);
    grid.insert(0, vec!['.'; grid[0].len()]);
    grid
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

fn traverse(
    current_cordinate: Cordinate,
    grid: &Grid,
    seen: &mut HashMap<Cordinate, usize>,
    current_area: &mut HashMap<Cordinate, usize>,
) {
    if seen.contains_key(&current_cordinate) {
        return ();
    }
    let mut sides = 4;
    let current_value = grid[current_cordinate[0]][current_cordinate[1]];
    let directions = vec![[1, 0], [-1, 0], [0, 1], [0, -1]];
    seen.insert(current_cordinate, sides);
    for direction in directions {
        let y = direction[0];
        let x = direction[1];
        if is_valid_index(current_cordinate, grid, y, x) {
            let new_y = (current_cordinate[0] as i32 + y) as usize;
            let new_x = (current_cordinate[1] as i32 + x) as usize;
            let next = grid[new_y][new_x];
            if next == current_value {
                traverse([new_y, new_x], grid, seen, current_area);
                sides -= 1
            }
        }
    }
    current_area.insert(current_cordinate, sides);
    seen.insert(current_cordinate, sides);
}

fn find_corners(
    start: Cordinate,
    retained: HashMap<Cordinate, usize>,
    grid: &Grid,
    current_area: HashMap<Cordinate, usize>,
) -> u32 {
    let mut result = HashMap::new();
    let current = grid[start[0]][start[1]];
    let mut grid_clone = grid.clone();
    if current == '.' {
        return 0;
    }
    for (edge, _) in retained {
        let mut sum = 0;
        if grid[edge[0] + 1][edge[1]] != current {
            if grid[edge[0]][edge[1] + 1] != current {
                *result.entry([edge[0], edge[1] + 1]).or_insert(0) += 1;
            }
            if grid[edge[0]][edge[1] - 1] != current {
                *result.entry([edge[0] + 1, edge[1]]).or_insert(0) += 1;
            }
        }
        if grid[edge[0] - 1][edge[1]] != current {
            if grid[edge[0]][edge[1] + 1] != current {
                *result.entry([edge[0] - 1, edge[1]]).or_insert(0) += 1;
            }
            if grid[edge[0]][edge[1] - 1] != current {
                *result.entry([edge[0], edge[1] - 1]).or_insert(0) += 1;
            }
        }
        if grid[edge[0] - 1][edge[1]] == current || grid[edge[0] + 1][edge[1]] == current {
            let r_d = current_area.contains_key(&[edge[0] + 1, edge[1] + 1]);
            let r_u = current_area.contains_key(&[edge[0] - 1, edge[1] + 1]);
            let l_d = current_area.contains_key(&[edge[0] + 1, edge[1] - 1]);
            let l_u = current_area.contains_key(&[edge[0] - 1, edge[1] - 1]);
            if r_u
                && grid[edge[0] - 1][edge[1] + 1] == current
                && grid[edge[0]][edge[1] + 1] != current
                && grid[edge[0] - 1][edge[1]] == current
            {
                *result.entry([edge[0], edge[1] + 1]).or_insert(0) += 1;
            }
            if l_u
                && grid[edge[0] - 1][edge[1] - 1] == current
                && grid[edge[0]][edge[1] - 1] != current
                && grid[edge[0] - 1][edge[1]] == current
            {
                *result.entry([edge[0], edge[1] - 1]).or_insert(0) += 1;
            }
            if r_d
                && grid[edge[0] + 1][edge[1] + 1] == current
                && grid[edge[0] + 1][edge[1]] == current
                && grid[edge[0]][edge[1] + 1] != current
            {
                *result.entry([edge[0], edge[1] + 1]).or_insert(0) += 1;
            }
            if l_d
                && grid[edge[0] + 1][edge[1] - 1] == current
                && grid[edge[0] + 1][edge[1]] == current
                && grid[edge[0]][edge[1] - 1] != current
            {
                *result.entry([edge[0], edge[1] - 1]).or_insert(0) += 1;
            }
        }
    }
    for (cordinate, val) in &result {
        grid_clone[cordinate[0]][cordinate[1]] = char::from_digit(*val, 10).unwrap();
    }
    // for line in grid_clone {
    //     println!("{:?}", line);
    // }
    result.into_values().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        let result = solve(input);
        assert_eq!(result, 80);
    }
    #[test]
    fn test_2() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let result = solve(input);
        assert_eq!(result, 236);
    }
    #[test]
    fn test_3() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        let result = solve(input);
        assert_eq!(result, 368);
    }
}
