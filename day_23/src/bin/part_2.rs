use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let result = find_largest_clique(input);
    println!("{}", result);
}

fn parse_graph(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split('-');
        let (a, b) = (parts.next().unwrap(), parts.next().unwrap());
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }

    graph
}

fn find_largest_clique(input: &str) -> String {
    let graph = parse_graph(input);
    let mut max_clique = HashSet::new();

    for &start_node in graph.keys() {
        let mut current = HashSet::from([start_node]);
        let mut candidates: HashSet<_> = graph[start_node].iter().copied().collect();

        while let Some(&next) = candidates.iter().next() {
            let is_connected_to_all = current.iter().all(|&node| graph[node].contains(next));

            if is_connected_to_all {
                current.insert(next);
                candidates = candidates.intersection(&graph[next]).copied().collect();
            } else {
                candidates.remove(next);
            }
        }

        if current.len() > max_clique.len() {
            max_clique = current;
        }
    }

    let mut result: Vec<_> = max_clique.into_iter().collect();
    result.sort_unstable();
    result.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "kh-tc
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
        assert_eq!(find_largest_clique(input), "co,de,ka,ta");
    }
}
