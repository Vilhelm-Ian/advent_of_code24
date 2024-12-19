fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

type Grid = Vec<Vec<char>>;
type Instructions = Vec<char>;

fn parse(input: &str) -> (Grid, Instructions) {
    let mut grid = vec![];
    let mut instructions = vec![];
    input.lines().for_each(|line| {
        let chars: Vec<char> = line.chars().collect();
        if !chars.is_empty() && chars[0] == '#' {
            grid.push(chars);
        } else {
            instructions.extend_from_slice(&chars);
        };
    });
    (grid, instructions)
}

type Cordinate = [usize; 2];
type Direction = [i32; 2];

fn find_robot(grid: &Grid) -> Cordinate {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                return [y, x];
            }
        }
    }
    panic!("robot not found")
}

fn match_movement(instruction: char) -> Direction {
    match instruction {
        '^' => [-1, 0],
        'v' => [1, 0],
        '>' => [0, 1],
        '<' => [0, -1],
        _ => panic!("not valid movement"),
    }
}

fn update_cordinate(current_cordinate: Cordinate, direction: Direction) -> Cordinate {
    [
        (current_cordinate[0] as i32 + direction[0]) as usize,
        (current_cordinate[1] as i32 + direction[1]) as usize,
    ]
}

fn index_grid(cordinate: Cordinate, grid: &Grid) -> char {
    grid[cordinate[0]][cordinate[1]]
}

fn move_element(
    grid: &mut Grid,
    instruction: char,
    current_cordinate: Cordinate,
) -> Option<Cordinate> {
    let current_square = index_grid(current_cordinate, grid);
    let direction = match_movement(instruction);
    let next_cordinate = update_cordinate(current_cordinate, direction);
    let next_square = index_grid(next_cordinate, grid);
    if next_square == '#'
        || (next_square != '.' && move_element(grid, instruction, next_cordinate).is_none())
    {
        return None;
    }
    grid[next_cordinate[0]][next_cordinate[1]] = current_square;
    grid[current_cordinate[0]][current_cordinate[1]] = '.';
    Some(next_cordinate)
}

fn calculate_score(grid: &Grid) -> i32 {
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                result += 100 * y + x;
            }
        }
    }
    result as i32
}

pub fn solve(input: &str) -> i32 {
    let (mut grid, instructions) = parse(input);
    let mut current_cordinate = find_robot(&grid);
    for instruction in instructions {
        let new_cordiante = move_element(&mut grid, instruction, current_cordinate);
        if let Some(new_cordiante) = new_cordiante {
            current_cordinate = new_cordiante;
        }
    }
    calculate_score(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const INPUT_BIG: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
    pub const INPUT_SMALL: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    #[test]
    fn test_1() {
        let result = solve(INPUT_BIG);
        assert_eq!(result, 10092);
    }

    #[test]
    fn test_2() {
        let result = solve(INPUT_SMALL);
        assert_eq!(result, 2028);
    }
}
