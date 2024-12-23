use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

fn parse(input: &str) -> Vec<[&str; 2]> {
    input
        .lines()
        .map(|line| {
            let mut line = line.split('-');
            let a = line.next().unwrap();
            let b = line.next().unwrap();
            [a, b]
        })
        .collect()
}

pub fn solve(input: &str) -> i32 {
    let pairs = parse(input);
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for pair in pairs {
        (*graph.entry(pair[0]).or_default()).insert(pair[1]);
        (*graph.entry(pair[1]).or_default()).insert(pair[0]);
    }
    let mut seen = HashSet::new();
    let mut result = HashSet::new();
    for (node_1, nodes_1) in graph.iter() {
        seen.insert(node_1);
        for node_2 in nodes_1.iter() {
            if seen.contains(node_2) {
                continue;
            }
            let nodes_2 = graph.get(node_2).unwrap();
            let mut intersection: Vec<&&str> = nodes_2.intersection(nodes_1).collect();
            for node_3 in intersection {
                if node_1.chars().nth(0).unwrap() != 't'
                    && node_2.chars().nth(0).unwrap() != 't'
                    && node_3.chars().nth(0).unwrap() != 't'
                {
                    continue;
                }
                let mut m = vec![node_1, node_2, node_3];
                m.sort();
                result.insert(m);
            }
        }
    }
    result.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    #[test]
    fn it_works() {
        let result = solve(INPUT);
        assert_eq!(result, 7);
    }
}
