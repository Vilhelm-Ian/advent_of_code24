fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

fn parse(input: &str) -> (Vec<[i32; 5]>, Vec<[i32; 5]>) {
    let mut locks = vec![];
    let mut keys = vec![];
    input.split("\n\n").for_each(|grid| {
        let mut is_lock = false;
        let mut result = [0, 0, 0, 0, 0];
        grid.lines().into_iter().enumerate().for_each(|(y, line)| {
            if y == 0 && line == "#####" {
                is_lock = true;
            }
            for (i, char) in line.chars().enumerate() {
                if char == '#' && ((is_lock && y != 0) || (!is_lock && y != 6)) {
                    result[i] += 1;
                }
            }
        });
        if is_lock {
            locks.push(result);
        } else {
            keys.push(result)
        }
    });

    (keys, locks)
}

fn solve(input: &str) -> i32 {
    let (mut keys, mut locks) = parse(input);
    // println!("{:?}", l);
    // for lock in l.0 {
    //     println!("{:?}", lock);
    // }
    let mut result = 0;
    for key in keys {
        locks.iter().enumerate().for_each(|(i, lock)| {
            if check_if_valid(key, *lock) {
                result += 1;
            }
        });
    }
    result
}

fn check_if_valid(key: [i32; 5], lock: [i32; 5]) -> bool {
    for i in 0..5 {
        if key[i] + lock[i] > 5 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
    #[test]
    fn it_works() {
        let result = solve(INPUT);
        assert_eq!(result, 3);
    }
}
