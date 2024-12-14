use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use std::rc::Rc;

use regex::Regex;

#[derive(PartialEq, Eq, Clone)]
struct Robot {
    position: Cordinate,
    velocity: Velocity,
}

impl Robot {
    fn move_robot(&mut self, grid_height: usize, grid_width: usize) {
        let mut new_positiony = self.position[0] as i32 + self.velocity[0];
        let mut new_positionx = self.position[1] as i32 + self.velocity[1];
        if new_positiony < 0 {
            new_positiony = grid_height as i32 + new_positiony;
        }
        if new_positionx < 0 {
            new_positionx = grid_width as i32 + new_positionx;
        }
        if new_positiony >= grid_height as i32 {
            new_positiony = new_positiony % grid_height as i32;
        }
        if new_positionx >= grid_width as i32 {
            new_positionx = new_positionx % grid_width as i32;
        }
        self.position[0] = new_positiony as usize;
        self.position[1] = new_positionx as usize;
    }

    fn move_times(&mut self, n: i32, grid_height: usize, grid_width: usize) {
        let y_offset = (self.position[0] as i32 + self.velocity[0] * n) % grid_height as i32;
        let x_offset = (self.position[1] as i32 + self.velocity[1] * n) % grid_width as i32;
        if self.velocity[0] >= 0 || y_offset == 0 {
            self.position[0] = y_offset as usize;
        } else {
            self.position[0] = (grid_height as i32 + y_offset) as usize;
        }
        if self.velocity[1] >= 0 || x_offset == 0 {
            self.position[1] = x_offset as usize
        } else {
            self.position[1] = (grid_width as i32 + x_offset) as usize;
        }
    }

    fn new(position: Cordinate, velocity: Velocity) -> Self {
        Self { position, velocity }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input, 101, 103, 100);
    println!("{:?}", result);
}

type Cordinate = [usize; 2];
type Velocity = [i32; 2];

fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"-?\d+").unwrap();
    let robots = input
        .lines()
        .map(|line| {
            let mut cordinates = vec![];
            let mut velocities = vec![];
            re.find_iter(line).enumerate().for_each(|(i, number)| {
                if i > 1 {
                    let number = number.as_str().parse::<i32>().unwrap();
                    velocities.push(number)
                } else {
                    let number = number.as_str().parse::<usize>().unwrap();
                    cordinates.push(number)
                }
            });
            Robot::new(
                [cordinates[1], cordinates[0]],
                [velocities[1], velocities[0]],
            )
        })
        .collect();
    robots
}

fn solve(input: &str, width: usize, height: usize, iterations: i32) -> i32 {
    let mut robots = parse(input);
    let mut robots_clone = robots.clone();
    // let mut robots_clone2: Vec = robots.iter().map(|robot| Rc::new(RefCell::new(robot))).collect();
    // for robot in robots.iter_mut() {
    //     robot.move_times(iterations, height, width)
    // }
    let mut meet = false;
    let mut i = 1;
    loop {
        for robot in robots.iter_mut() {
            robot.move_robot(height, width);
        }
        if i % 2 == 0 && !meet {
            for robot in robots_clone.iter_mut() {
                robot.move_robot(height, width);
            }
        }
        if robots_clone.iter().eq(&robots) {
            if meet {
                println!("{i}");
                println!("cycel detecetded");
                break;
            }
            meet = true;
            i = 1;
        }
        print_grid(width, height, &robots, i);
        // std::thread::sleep(std::time::Duration::from_millis(200));
        // print!("\x1b[2j\x1b[h");
        i += 1;
    }
    fill_quadrants(height, width, &robots)
}

fn fill_quadrants(grid_height: usize, grid_width: usize, robots: &Vec<Robot>) -> i32 {
    let mut first_quardant = 0;
    let mut second_quardant = 0;
    let mut third_quardant = 0;
    let mut fourth_quardant = 0;
    for robot in robots {
        if robot.position[0] < grid_height / 2 {
            if robot.position[1] < grid_width / 2 {
                first_quardant += 1;
            }
            if robot.position[1] > grid_width / 2 {
                second_quardant += 1;
            }
        }
        if robot.position[0] > grid_height / 2 {
            if robot.position[1] < grid_width / 2 {
                third_quardant += 1;
            }
            if robot.position[1] > grid_width / 2 {
                fourth_quardant += 1;
            }
        }
    }
    // println!("{first_quardant} {second_quardant} {third_quardant} {fourth_quardant}");
    first_quardant * second_quardant * third_quardant * fourth_quardant
}

fn print_grid(width: usize, height: usize, robots: &Vec<Robot>, i: usize) {
    let row = vec!['.'; width];
    let mut grid = vec![row; height];
    for robot in robots {
        let [y, x] = robot.position;
        grid[y][x] = '#'
    }
    let mut triangle = false;
    for y in 0..grid.len() - 3 {
        for x in 3..grid[0].len() - 3 {
            if grid[y][x] != '#' || grid[y + 1][x + 1] != '#' || grid[y + 1][x - 1] != '#' {
                continue;
            }
            if grid[y + 1][x + 1] != '#' || grid[y + 1][x - 1] != '#' {
                continue;
            }
            if grid[y + 2][x + 2] != '#' || grid[y + 2][x - 2] != '#' {
                continue;
            }
            if grid[y + 3][x + 3] != '#' || grid[y + 3][x - 3] != '#' {
                continue;
            }
            grid[y][x] = 'O';
            triangle = true;
        }
    }
    if triangle {
        for line in grid.iter() {
            let line: String = line.iter().collect();
            println!("{:?}", line);
        }
        println!("{i}");
    } // map[0][51] = 'm';
      // println!("\n\n");
      // map[1][50] = 'm';
      // map[1][51] = 'm';
      // map[1][52] = 'm';
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    #[test]
    fn it_works() {
        let result = solve(INPUT, 11, 7, 100);
        assert_eq!(result, 12);
    }
    // #[test]
    // fn it_works() {
    //     let input = "p=2,4 v=2,-3";
    //     let result = solve(input);
    //     assert_eq!(result[0].position, [3, 1]);
    // }
    #[test]
    fn move_n_times() {
        let mut robot1 = Robot::new([3, 1], [-1, -4]);
        let mut robot2 = Robot::new([3, 1], [-1, -4]);
        for _ in 0..11 {
            robot1.move_robot(10, 10);
        }
        robot2.move_times(11, 10, 10);
        assert_eq!(robot1.position, robot2.position);
    }
}
