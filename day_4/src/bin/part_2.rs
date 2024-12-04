fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{result}");
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve(input: &str) -> i32 {
    let grid = parse(input);
    let mut count = 0;
    for y in 0..grid.len() - 2 {
        for x in 0..grid[0].len() {
            if check_diagonal_right(&grid, y, x) && check_diagonal_left(&grid, y, x + 2) {
                count += 1;
            }
        }
    }
    count
}


fn check_diagonal_right(grid: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let mut result = ['a', 'a', 'a'];
    if grid[0].len() as i32 - 3 - x as i32 >= 0 && grid.len() as i32 - 3 - y as i32 >= 0 {
        for i in 0..3 {
            result[i] = grid[y + i][x + i];
        }
    }
    result == ['M', 'A', 'S'] || result == ['S', 'A', 'M']
}

fn check_diagonal_left(grid: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let mut result = ['a', 'a', 'a'];
    if x >= 2 && grid.len() as i32 - 3 - y as i32 >= 0 {
        for i in 0..3 {
            result[i] = grid[y + i][x - i];
        }
    }
    result == ['M', 'A', 'S'] || result == ['S', 'A', 'M']
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
        assert_eq!(result, 9);
    }
}
