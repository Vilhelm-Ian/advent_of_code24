use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    //1539130 too high
    //1523849 too low
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

fn duplicate_map(grid: &Grid) -> Grid {
    let mut map = vec![];
    for y in 0..grid.len() {
        let mut row = vec![];
        for x in 0..grid[0].len() {
            match grid[y][x] {
                '#' => {
                    row.extend_from_slice(&['#'; 2]);
                }
                '.' => {
                    row.extend_from_slice(&['.'; 2]);
                }
                'O' => {
                    row.extend_from_slice(&['[', ']']);
                }
                '@' => {
                    row.extend_from_slice(&['@', '.']);
                }
                _ => {
                    panic!("not a valid char");
                }
            }
        }
        map.push(row);
    }
    map
}

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
    let og_grid = grid.clone();
    let current_square = index_grid(current_cordinate, grid);
    let direction = match_movement(instruction);
    let next_cordinate = update_cordinate(current_cordinate, direction);
    let next_square = index_grid(next_cordinate, grid);
    let mut next_cordinate_2 = None;
    let mut current_square_2 = None;
    let mut current_cordinate_2 = None;
    let mut next_square_2 = None;
    if matches!(instruction, 'v' | '^') {
        current_cordinate_2 = match current_square {
            ']' => Some(update_cordinate(current_cordinate, [0, -1])),
            '[' => Some(update_cordinate(current_cordinate, [0, 1])),
            _ => None,
        };
        if current_cordinate_2.is_some() {
            current_square_2 = Some(index_grid(current_cordinate_2.unwrap(), grid));
            next_cordinate_2 = Some(update_cordinate(current_cordinate_2.unwrap(), direction));
            next_square_2 = Some(index_grid(next_cordinate_2.unwrap(), grid));
            if next_square_2.unwrap() == '#' {
                return None;
            }
        }
    }
    if next_square == '#'
        || (next_square != '.' && move_element(grid, instruction, next_cordinate).is_none())
    {
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                grid[y][x] = og_grid[y][x];
            }
        }
        return None;
    }
    if let Some(next_cordinate_2) = next_cordinate_2 {
        if matches!(
            (next_square_2.unwrap(), current_square_2.unwrap()),
            ('[', ']') | (']', '[')
        ) && move_element(grid, instruction, next_cordinate_2).is_none()
        {
            for y in 0..grid.len() {
                for x in 0..grid[0].len() {
                    grid[y][x] = og_grid[y][x];
                }
            }
            return None;
        }
        grid[next_cordinate_2[0]][next_cordinate_2[1]] = current_square_2.unwrap();
        grid[current_cordinate_2.unwrap()[0]][current_cordinate_2.unwrap()[1]] = '.';
    };
    grid[next_cordinate[0]][next_cordinate[1]] = current_square;
    grid[current_cordinate[0]][current_cordinate[1]] = '.';
    Some(next_cordinate)
}

fn calculate_score(grid: &Grid) -> i32 {
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '[' {
                result += 100 * y + x;
            }
        }
    }
    result as i32
}

pub fn solve(input: &str) -> i32 {
    let (mut grid, instructions) = parse(input);
    let mut grid = duplicate_map(&grid);
    let mut current_cordinate = find_robot(&grid);
    let mut set = HashSet::new();
    for instruction in instructions {
        let new_cordiante = move_element(&mut grid, instruction, current_cordinate);

        if let Some(new_cordiante) = new_cordiante {
            current_cordinate = new_cordiante;
        } else {
            // println!("{instruction}");
            let grid_clone = grid.clone();
            if set.contains(&grid_clone) {
                continue;
            }
            set.insert(grid_clone);
            // for (i, mut line) in grid.iter().enumerate() {
            //     let mut l = line.clone();
            //     if line.iter().any(|l| *l == '@') {
            //         l.extend_from_slice(&['.', '.', instruction]);
            //     }
            //     l.insert(0, char::from_digit(i as u32 % 10, 10).unwrap());
            //     let l: String = l.iter().collect();
            //     println!("{:?}", l);
            // }
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
    pub const INPUT_SMALL: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    pub const SMALL_RESULT: &str = "##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############";

    #[test]
    fn test_2_1() {
        let result = solve(INPUT_BIG);
        assert_eq!(result, 9021);
    }

    #[test]
    fn test_2_2() {
        let (grid, instructions) = parse(INPUT_SMALL);
        let mut grid = duplicate_map(&grid);
        let mut current_cordinate = find_robot(&grid);
        for instruction in instructions {
            if let Some(cordinate) = move_element(&mut grid, instruction, current_cordinate) {
                current_cordinate = cordinate;
            }
        }
        let result: String = grid
            .iter()
            .map(|line| line.iter().collect::<String>())
            .reduce(|acc, e| acc + "\n" + e.as_str())
            .unwrap();
        println!("{result}");
        assert_eq!(result, SMALL_RESULT);
    }
}
