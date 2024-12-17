use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    let mut print = result
        .iter()
        .map(|num| char::from_digit(*num as u32, 10).unwrap().to_string() + ",")
        .collect::<String>();

    print.pop();
    println!("{print}");
}

fn solve(input: &str) -> Vec<u64> {
    let (mut registers, instructions) = parse(input);
    let mut i = 0;
    let mut result = vec![];
    loop {
        if i >= instructions.len() {
            break;
        }
        let instruction = instructions[i];
        call_instruction(&mut registers, &instructions, &mut i, &mut result);
        if instruction != 3 {
            i += 2
        }
    }
    result
}

fn call_instruction(
    registers: &mut [u64; 3],
    instructions: &Vec<u64>,
    i: &mut usize,
    result: &mut Vec<u64>,
) {
    let instruction = instructions[*i];
    match instruction {
        0 => adv(registers, instructions[*i + 1]),
        1 => bxl(registers, instructions[*i + 1]),
        2 => bst(registers, instructions[*i + 1]),
        3 => jnz(registers, instructions[*i + 1], i),
        4 => bxc(registers, instructions[*i + 1]),
        5 => out(registers, instructions[*i + 1], result),
        6 => bdv(registers, instructions[*i + 1]),
        7 => cdv(registers, instructions[*i + 1]),
        _ => panic!("not valid instruction"),
    };
}

fn parse(input: &str) -> ([u64; 3], Vec<u64>) {
    let re = Regex::new(r"\d+").unwrap();
    let mut registers = [0, 0, 0];
    let mut instructions = vec![];
    for (i, line) in input.lines().enumerate() {
        if i < 3 {
            registers[i] = re
                .find_iter(line)
                .next()
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
        }
        if i > 3 {
            instructions = re
                .find_iter(line)
                .map(|num| num.as_str().parse::<u64>().unwrap())
                .collect();
        }
    }
    (registers, instructions)
}

fn adv(registers: &mut [u64; 3], operand: u64) {
    let b = if operand > 3 {
        registers[(operand % 4) as usize]
    } else {
        operand
    };
    registers[0] /= 2_u64.pow(b as u32);
}

fn bxl(registers: &mut [u64; 3], operand: u64) {
    registers[1] ^= operand;
}

fn bst(registers: &mut [u64; 3], operand: u64) {
    let b = if operand > 3 {
        registers[(operand % 4) as usize]
    } else {
        operand
    };
    registers[1] = b % 8;
}

fn jnz(registers: &mut [u64; 3], operand: u64, i: &mut usize) {
    if registers[0] == 0 {
        *i += 2;
        return;
    }
    *i = operand as usize;
}

fn bxc(registers: &mut [u64; 3], operand: u64) {
    registers[1] = registers[1] ^ registers[2];
}

fn out(registers: &mut [u64; 3], operand: u64, result: &mut Vec<u64>) {
    let b = if operand > 3 {
        registers[(operand % 4) as usize]
    } else {
        operand
    };
    let output = b % 8;
    (*result).push(output)
}

fn bdv(registers: &mut [u64; 3], operand: u64) {
    let b = if operand > 3 {
        registers[(operand % 4) as usize]
    } else {
        operand
    };
    registers[1] = registers[0] / 2_u64.pow(b as u32);
}

fn cdv(registers: &mut [u64; 3], operand: u64) {
    let b = if operand > 3 {
        registers[(operand % 4) as usize]
    } else {
        operand
    };
    registers[2] = registers[0] / 2_u64.pow(b as u32);
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn it_works() {
        let result = solve(INPUT);
        assert_eq!(result, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }
    #[test]
    fn test_1() {
        let mut registers = [0, 0, 9];
        let mut result = vec![];
        let instructions = vec![2, 6];
        let mut i = 0;
        call_instruction(&mut registers, &instructions, &mut i, &mut result);
        assert_eq!(registers[1], 1);
    }
    #[test]
    fn test_2() {
        let mut registers = [10, 0, 0];
        let mut result = vec![];
        let instructions = vec![5, 0, 5, 1, 5, 4];
        let mut i = 0;
        loop {
            if i >= instructions.len() - 1 {
                break;
            }
            let instruction = instructions[i];
            call_instruction(&mut registers, &instructions, &mut i, &mut result);
            if instruction != 3 {
                i += 2
            }
        }
        assert_eq!(result, vec![0, 1, 2]);
    }
    #[test]
    fn test_3() {
        let mut registers = [2024, 0, 0];
        let mut result = vec![];
        let instructions = vec![0, 1, 5, 4, 3, 0];
        let mut i = 0;
        loop {
            if i >= instructions.len() - 1 {
                break;
            }
            let instruction = instructions[i];
            call_instruction(&mut registers, &instructions, &mut i, &mut result);
            if instruction != 3 {
                i += 2
            }
        }
        assert_eq!(result, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(registers[0], 0);
    }
    #[test]
    fn test_4() {
        let mut registers = [0, 29, 0];
        let mut result = vec![];
        let instructions = vec![1, 7];
        let mut i = 0;
        loop {
            if i >= instructions.len() - 1 {
                break;
            }
            let instruction = instructions[i];
            call_instruction(&mut registers, &instructions, &mut i, &mut result);
            if instruction != 3 {
                i += 2
            }
        }
        assert_eq!(registers[1], 26);
    }
    #[test]
    fn test_5() {
        let mut registers = [0, 2024, 43690];
        let mut result = vec![];
        let instructions = vec![4, 0];
        let mut i = 0;
        loop {
            if i >= instructions.len() - 1 {
                break;
            }
            let instruction = instructions[i];
            call_instruction(&mut registers, &instructions, &mut i, &mut result);
            if instruction != 3 {
                i += 2
            }
        }
        assert_eq!(registers[1], 44354);
    }
}
