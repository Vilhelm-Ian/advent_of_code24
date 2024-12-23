use std::collections::hash_map::RandomState;
use std::collections::{btree_set::Intersection, HashMap, HashSet};

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

pub fn solve(input: &str) -> String {
    let pairs = parse(input);
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for pair in pairs {
        (*graph.entry(pair[0]).or_default()).insert(pair[1]);
        (*graph.entry(pair[1]).or_default()).insert(pair[0]);
    }
    let mut seen = HashSet::new();
    let mut result = vec![];
    for (key, nodes) in graph.iter() {
        let a = traverse(
            key,
            &seen,
            &graph,
            vec![],
            &nodes.intersection(nodes).copied().collect(),
        );
        result.push(a);
        seen.insert(key);
    }
    result
        .into_iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
}

fn traverse<'a, 'b>(
    node: &'a str,
    seen: &HashSet<&str>,
    graph: &'a HashMap<&str, HashSet<&str>>,
    mut result: Vec<&'a str>,
    intersection: &HashSet<&str>,
) -> String {
    let mut seen = seen.clone();
    if seen.contains(node) {
        return node.to_string();
    }
    result.push(node);
    result.sort();
    let key = result.iter().fold(String::new(), |acc, b| acc + *b);
    if intersection.is_empty() {
        return key;
    }
    let mut answer = vec![];

    for node_2 in intersection.iter() {
        let nodes_2 = graph.get(*node_2).unwrap();
        let new_intersection = nodes_2.intersection(intersection).copied().collect();
        answer.push(traverse(
            node_2,
            &seen,
            graph,
            result.clone(),
            &new_intersection,
        ));
        seen.insert(node_2);
    }
    answer
        .into_iter()
        .max_by(|m, b| m.len().cmp(&b.len()))
        .unwrap()
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
    fn part_2() {
        let result = solve(INPUT);
        assert_eq!(result, "codekata".to_string());
    }
}
