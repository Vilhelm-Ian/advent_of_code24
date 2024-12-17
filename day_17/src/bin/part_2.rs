use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{result}");
}

fn solve(input: &str) -> u64 {
    let (mut registers, instructions) = parse(input);
    registers[0] = 8_u64.pow(instructions.len() as u32 - 1);
    let mut result = vec![];
    loop {
        let mut i = 0;
        if result.eq(&instructions) {
            break;
        }
        let prev = registers[0];
        result = vec![];
        loop {
            for i in 0..result.len() {
                if result[i] != instructions[i] {
                    break;
                }
            }
            if i >= instructions.len() {
                break;
            }
            let instruction = instructions[i];
            call_instruction(&mut registers, &instructions, &mut i, &mut result);
            if instruction != 3 {
                i += 2
            }
        }
        let mut mistakes = false;
        if result.len() > instructions.len() {
            println!("too long");
            break;
        }
        for i in 0..instructions.len() {
            if result[i] != instructions[i] {
                registers[0] = prev + 8_u64.pow(i as u32);
                mistakes = true;
            }
        }
        if !mistakes {
            return prev;
        }
    }
    panic!("can't come here");
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

