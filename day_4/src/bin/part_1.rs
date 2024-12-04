fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve(input: &str) -> i32 {
    let grid = parse(input);
    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            check_vertical(&grid, y, x, &mut count);
            check_horizontal(&grid, y, x, &mut count);
            check_diagonal_right(&grid, y, x, &mut count);
            check_diagonal_left(&grid, y, x, &mut count);
        }
    }
    count
}

fn check_vertical(grid: &Vec<Vec<char>>, y: usize, x: usize, count: &mut i32) {
    let mut result = ['a', 'a', 'a', 'a'];
    if grid.len() as i32 - y as i32 - 4 >= 0 {
        for i in 0..4 {
            result[i] = grid[y + i][x];
        }
    }
    if result == ['X', 'M', 'A', 'S'] || result == ['S', 'A', 'M', 'X'] {
        *count += 1;
    }
}

fn check_horizontal(grid: &Vec<Vec<char>>, y: usize, x: usize, count: &mut i32) {
    let mut result = ['a', 'a', 'a', 'a'];
    if grid[0].len() as i32 - x as i32 - 4 >= 0 {
        for i in 0..4 {
            result[i] = grid[y][x + i];
        }
    }
    if result == ['X', 'M', 'A', 'S'] || result == ['S', 'A', 'M', 'X'] {
        *count += 1;
    }
}

fn check_diagonal_right(grid: &Vec<Vec<char>>, y: usize, x: usize, count: &mut i32) {
    let mut result = ['a', 'a', 'a', 'a'];
    if grid[0].len() as i32 - 4 - x as i32 >= 0 && grid.len() as i32 - 4 - y as i32 >= 0 {
        for i in 0..4 {
            result[i] = grid[y + i][x + i];
        }
    }
    if result == ['X', 'M', 'A', 'S'] || result == ['S', 'A', 'M', 'X'] {
        *count += 1;
    }
}

fn check_diagonal_left(grid: &Vec<Vec<char>>, y: usize, x: usize, count: &mut i32) {
    let mut result = ['a', 'a', 'a', 'a'];
    if x >= 3 && grid.len() as i32 - 4 - y as i32 >= 0 {
        for i in 0..4 {
            result[i] = grid[y + i][x - i];
        }
    }
    if result == ['X', 'M', 'A', 'S'] || result == ['S', 'A', 'M', 'X'] {
        *count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = solve(input);
        assert_eq!(result, 18);
    }
}
