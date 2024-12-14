use regex::Regex;

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
            println!("x: {new_positionx}");
            new_positionx = grid_width as i32 + new_positionx;
            println!("x: {new_positionx}");
        }
        if new_positiony >= grid_height as i32 {
            new_positiony = new_positiony % grid_height as i32;
        }
        if new_positionx >= grid_width as i32 {
            new_positionx = new_positionx % grid_width as i32;
        }
        self.position[0] = new_positiony as usize;
        self.position[1] = new_positionx as usize;
        println!("{:?}", self.position);
    }

    fn move_times(&mut self, n: i32, grid_height: usize, grid_width: usize) {
        if self.velocity[0] >= 0 {
            self.position[0] =
                ((self.position[0] as i32 + self.velocity[0] * n) % grid_height as i32) as usize;
        } else {
            self.position[0] = (grid_height as i32
                + ((self.position[0] as i32 + self.velocity[0] * n) % grid_height as i32))
                as usize;
        }
        if self.velocity[1] >= 0 {
            self.position[1] =
                ((self.position[1] as i32 + self.velocity[1] * n) % grid_width as i32) as usize;
        } else {
            self.position[1] = (grid_width as i32
                + ((self.position[1] as i32 + self.velocity[1] * n) % grid_width as i32))
                as usize;
        }
    }

    fn new(position: Cordinate, velocity: Velocity) -> Self {
        Self { position, velocity }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    // println!("{:?}", result);
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
            println!("{:?}", cordinates);
            println!("{:?}", velocities);
            Robot::new(
                [cordinates[1], cordinates[0]],
                [velocities[1], velocities[0]],
            )
        })
        .collect();
    robots
}

fn solve(input: &str) -> Vec<Robot> {
    let mut robots = parse(input);
    for i in 0..5 {
        for robot in robots.iter_mut() {
            robot.move_robot(7, 11)
        }
    }
    robots
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn it_works() {
    //     let input = "p=2,4 v=2,-3";
    //     let result = solve(input);
    //     assert_eq!(result[0].position, [3, 1]);
    // }
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
