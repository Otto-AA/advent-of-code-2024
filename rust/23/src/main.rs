use std::{
    collections::HashSet,
    env::{self},
    fs::read_to_string,
};

use anyhow::{Context, Result};
use petgraph::prelude::UnGraphMap;

type Network<'a> = UnGraphMap<&'a str, ()>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .context("Please provide a path to the input file")?;

    let input = read_to_string(path)?;
    let network = parse_network(&input);

    println!("Part one: {}", part_one(&network));
    println!("Part two: {}", part_two(&network));

    Ok(())
}

fn parse_network(input: &str) -> Network<'_> {
    input
        .lines()
        .map(|line| {
            let mut parts: Vec<&str> = line.split("-").collect();
            parts.sort();
            (parts[0], parts[1])
        })
        .collect()
}

fn part_one(graph: &UnGraphMap<&str, ()>) -> usize {
    let mut found: HashSet<Vec<&str>> = HashSet::new();

    for (a, b, _) in graph.all_edges() {
        if !a.starts_with("t") && !b.starts_with("t") {
            continue;
        }

        let neighbours_a: HashSet<&str> = graph.neighbors(a).collect();
        let neighbours_b: HashSet<&str> = graph.neighbors(b).collect();

        for x in neighbours_a.intersection(&neighbours_b) {
            let mut entry = vec![a, b, *x];
            entry.sort();
            found.insert(entry);
        }
    }

    found.len()
}

fn part_two(network: &Network) -> String {
    let candidates: HashSet<String> = network.nodes().map(|s| s.to_string()).collect();
    let mut c = vec![];
    let clique = largest_clique(network, 0, &mut c, candidates.clone());
    let mut clique: Vec<String> = clique.into_iter().collect();
    clique.sort();
    clique.join(",")
}

fn largest_clique(
    network: &Network,
    min_size: usize,
    current_clique: &mut Vec<String>,
    candidates: HashSet<String>,
) -> Vec<String> {
    let mut min_size = min_size;
    let mut best_clique = current_clique.clone();

    let mut remaining_candidates = candidates.clone();

    // only recurse if we still can get better
    if min_size < current_clique.len() + candidates.len() {
        for c in candidates {
            // add c to current clique
            current_clique.push(c.clone());
            let next_candidates = get_candidates(network, &remaining_candidates, &c);
            let largest_clique_with_c =
                largest_clique(network, min_size, current_clique, next_candidates);
            if largest_clique_with_c.len() > min_size {
                min_size = largest_clique_with_c.len();
                best_clique = largest_clique_with_c;
            }
            // remove c
            current_clique.pop().unwrap();
            remaining_candidates.remove(&c);
        }
    }

    best_clique
}

fn get_candidates(
    network: &Network,
    previous_candidates: &HashSet<String>,
    added_candidate: &str,
) -> HashSet<String> {
    let neighbours: HashSet<String> = network
        .neighbors(added_candidate)
        .map(|s| s.to_string())
        .collect();
    previous_candidates
        .intersection(&neighbours)
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use anyhow::Result;

    use crate::{parse_network, part_one, part_two};

    #[test]
    fn part_one_sample() -> Result<()> {
        let input = read_to_string("sample.txt")?;
        let network = parse_network(&input);

        let solution = part_one(&network);

        assert_eq!(7, solution);
        Ok(())
    }

    #[test]
    fn part_two_sample() -> Result<()> {
        let input = read_to_string("sample.txt")?;
        let network = parse_network(&input);

        let solution = part_two(&network);

        assert_eq!("co,de,ka,ta", solution);
        Ok(())
    }
}
