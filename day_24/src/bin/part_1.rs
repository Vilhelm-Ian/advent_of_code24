use regex::Regex;
use std::collections::{BTreeMap, HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

fn parse(input: &str) -> (Vec<[&str; 2]>, Vec<[&str; 4]>) {
    let re = Regex::new(r"\w+").expect("invlid regex");
    let mut second = false;
    let mut start: Vec<[&str; 2]> = vec![];
    let mut instructions: Vec<[&str; 4]> = vec![];
    input.lines().for_each(|line| {
        if line.is_empty() {
            second = true;
            return ();
        }
        if !second {
            let mut split = line.split(": ");
            start.push([split.next().unwrap(), split.next().unwrap()]);
        } else {
            let mut matches = re.find_iter(line).map(|segment| segment.as_str());
            let temp = [
                matches.next().unwrap(),
                matches.next().unwrap(),
                matches.next().unwrap(),
                matches.next().unwrap(),
            ];
            instructions.push(temp)
        }
    });
    (start, instructions)
}

fn solve(input: &str) -> u64 {
    let (start, mut instructions) = parse(input);
    let mut map = BTreeMap::new();
    for [key, value] in start {
        map.insert(key, value.parse().unwrap());
    }
    while !instructions.is_empty() {
        let mut indexes_to_remove = vec![];
        for (i, [a, operation, b, target]) in instructions.iter().enumerate() {
            if map.contains_key(a) && map.contains_key(b) {
                let a = map.get(a).unwrap();
                let b = map.get(b).unwrap();
                let result = handle_operation(*a, operation, *b);
                map.insert(target, result);
                indexes_to_remove.push(i);
            }
        }
        // reverse sort
        indexes_to_remove.sort_by(|a, b| b.cmp(a));
        indexes_to_remove.iter().for_each(|i| {
            instructions.remove(*i);
        });
    }
    let mut result: u64 = 0;
    for (key, value) in map.iter().rev() {
        if !key.starts_with('z') {
            continue;
        }
        println!("{:?} {value}", key);
        result = result << 1;
        result = result | value;
        println!("{:b}", result);
    }
    println!("{:b}", result);
    result
}

fn handle_operation(a: u64, operation: &str, b: u64) -> u64 {
    match operation {
        "AND" => a & b,
        "OR" => a | b,
        "XOR" => a ^ b,
        _ => panic!("invaalid operation"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    #[test]
    fn it_works() {
        let result = solve(INPUT);
        assert_eq!(result, 2024);
    }
}
