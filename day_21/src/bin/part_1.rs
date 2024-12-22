use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::Iterator;
use std::ops::Deref;

fn main() {
    let input = include_str!("./input.txt");
    breath_first_search_real();
    // let result = solve(input);
    // println!("{:?}", result);
}

type Grid = Vec<Vec<char>>;
type Cordinate = [usize; 2];
type Direction = [i32; 2];

const NUMERIC_PAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

const ARROW_PAD: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve(input: &str) -> i32 {
    let codes = parse(input);
    0
}

fn breath_first_search_real() {
    let instructions = ['>', '<', '^', 'v', 'A'];
    let mut que = VecDeque::new();
    instructions
        .iter()
        .for_each(|instruction| que.push_front(vec![*instruction]));
    let target = vec![
        'v', '<', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^', 'A', 'A',
        '>', 'A', '<', 'v', 'A', 'A', 'A', '>', '^', 'A',
    ];
    let mut map = HashMap::new();
    let mut seen = HashSet::new();
    while let Some(commands) = que.pop_back() {
        if seen.contains(&commands) {
            continue;
        }
        seen.insert(commands.clone());
        if let Some(new_command) = read_command_to_pad(&commands, &mut map) {
            // println!("{:?}\n`{:?} \n\n", new_command, commands);
            if new_command.len() > target.len() {
                continue;
            }
            if new_command == target {
                // println!("{:?}", new_command);
                break;
            };
            let mut brk = false;
            for i in 0..new_command.len() {
                if target[i] != new_command[i] {
                    brk = true;
                    continue;
                }
            }
            if brk {
                continue;
            }
            instructions
                .iter()
                .filter(|i| !is_opposite(**i, *commands.last().unwrap_or(&' ')))
                .for_each(|instruction| {
                    let name_later = commands
                        .iter()
                        .chain(std::iter::once(instruction))
                        .copied()
                        .collect();
                    que.push_front(name_later);
                });
            // que.push()
        };
    }
}

fn is_opposite(a: char, b: char) -> bool {
    if a == '<' && b == '>' {
        true
    } else if b == '<' && a == '>' {
        true
    } else if a == 'v' && b == '^' {
        true
    } else if b == 'v' && a == '^' {
        true
    } else {
        false
    }
}

fn read_command_to_pad(
    command: &Vec<char>,
    seen: &mut HashMap<Vec<char>, (Vec<char>, Cordinate)>,
) -> Option<Vec<char>> {
    let mut current = [0, 2];
    let mut result = vec![];
    let mut z = 0;
    let mut visited = HashSet::new();
    for i in 1..command.len() {
        if let Some((key, (value, cordinate))) = seen.get_key_value(&command[0..i]) {
            current = *cordinate;
            result = value.clone();
            z = i;
        } else {
            break;
        }
    }
    for arrow in command {
        if visited.contains(&(current, result.len())) {
            // panic!("hat");
            return None;
        }
        visited.insert((current, result.len()));
        if *arrow == 'A' {
            result.push(ARROW_PAD[current[0]][current[1]]);
        }
        let direction = arrow_to_direction(arrow)?;
        current = update_cordinate(current, direction);
        // println!("arrow {:?} {:?} current {:?}", arrow, current, result);
        if current == [0, 0] || current[0] > 1 || current[1] > 2 {
            // println!("not approoved");
            return None;
        }
    }
    if !result.is_empty() {
        seen.insert(command.clone(), (result.clone(), current));
    }
    // println!("approoved");
    Some(result)
}

fn update_cordinate(current_cordinate: Cordinate, direction: Direction) -> Cordinate {
    [
        (current_cordinate[0] as i32 + direction[0]) as usize,
        (current_cordinate[1] as i32 + direction[1]) as usize,
    ]
}

fn direction_to_arrow(direction: &Direction) -> Option<char> {
    match direction {
        [0, 1] => Some('>'),
        [0, -1] => Some('<'),
        [-1, 0] => Some('^'),
        [1, 0] => Some('v'),
        [0, 0] => Some('A'),
        _ => None,
    }
}

fn arrow_to_direction(arrow: &char) -> Option<Direction> {
    match arrow {
        '>' => Some([0, 1]),
        '<' => Some([0, -1]),
        '^' => Some([-1, 0]),
        'v' => Some([1, 0]),
        'A' => Some([0, 0]),
        _ => None,
    }
}

fn arrow_to_cordinate(arrow: &char) -> Cordinate {
    match arrow {
        '>' => [1, 2],
        '<' => [1, 0],
        '^' => [0, 1],
        'v' => [1, 1],
        'A' => [0, 2],
        _ => panic!("not an arrow"),
    }
}
fn manhatan_distance(a: Cordinate, b: Cordinate) -> usize {
    ((a[0] as i32 - b[0] as i32).unsigned_abs() + (a[1] as i32 - b[1] as i32).unsigned_abs())
        as usize
}

fn code_to_index(code: char) -> Cordinate {
    match code {
        '9' => [0, 2],
        '8' => [0, 1],
        '7' => [0, 0],
        '6' => [1, 2],
        '5' => [1, 1],
        '4' => [1, 0],
        '3' => [2, 2],
        '2' => [2, 1],
        '1' => [2, 0],
        'A' => [3, 2],
        '0' => [3, 1],
        _ => panic!("not valid code"),
    }
}

fn direction_from_distances(a: Cordinate, b: Cordinate) -> Direction {
    [a[0] as i32 - b[0] as i32, a[1] as i32 - b[1] as i32]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 1;
        breath_first_search_real();
        assert_eq!(result, 4);
    }
}
