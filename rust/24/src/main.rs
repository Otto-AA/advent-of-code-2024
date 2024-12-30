use std::{collections::HashSet, env};

use anyhow::Result;
use gates::Gate;
use parser::{parse_input, CircuitGraph, Node};
use petgraph::{
    algo::toposort,
    dot::{Config, Dot},
    graph::EdgeReference,
    prelude::*,
    visit::{NodeRef, Walker},
};

mod gates;
mod parser;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("Please provide an input file path");
    let mut graph = parse_input(path)?;

    if args.len() > 2 {
        print_graph(&graph);
    } else {
        println!("Part one: {}", part_one(&mut graph));
        let mut graph = parse_input(path)?;
        part_two(&mut graph);
    }

    Ok(())
}

fn print_graph(graph: &CircuitGraph) {
    println!(
        "{:?}",
        Dot::with_attr_getters(
            graph,
            &[Config::NodeNoLabel, Config::EdgeNoLabel],
            &|_, edge| format!("label={:?}", edge.weight()),
            &|_, node| format!("label={}", node.weight().name.clone())
        )
    );
}

fn part_one(graph: &mut CircuitGraph) -> u64 {
    let sorted = toposort(&*graph, None).expect("Graph can be topologically sorted");

    for index in sorted {
        let node = &graph[index];
        if node.value.is_some() {
            // this element already has a computed value
            continue;
        }
        let edges: Vec<(&Gate, NodeIndex)> = graph
            .edges_directed(index, Incoming)
            .map(|edge| (edge.weight(), edge.source()))
            .collect();
        assert_eq!(2, edges.len());
        assert_eq!(edges[0].0, edges[1].0);
        let gate = edges[0].0;
        let in_left = graph[edges[0].1].value.unwrap();
        let in_right = graph[edges[1].1].value.unwrap();
        let result = gate.calculate(in_left, in_right);

        let node = &mut graph[index];
        node.value = Some(result);
    }

    get_result(graph)
}

fn get_result(graph: &CircuitGraph) -> u64 {
    let mut output_nodes: Vec<&Node> = graph
        .node_weights()
        .filter(|node| node.name.starts_with("z"))
        .collect();
    output_nodes.sort_by_key(|node| &node.name);
    output_nodes.reverse();

    let outputs: String = output_nodes
        .into_iter()
        .map(|node| node.value.expect("Output should be calculated"))
        .map(|b| match b {
            true => "1".to_string(),
            false => "0".to_string(),
        })
        .collect();
    u64::from_str_radix(&outputs, 2).expect(&format!("Can convert <{outputs}> to u64"))
}

fn part_two(graph: &mut CircuitGraph) {
    // We verify some properties and output violations
    // which we can then manually examine with the graph
    let input_indexes: HashSet<NodeIndex> = graph
        .node_indices()
        .filter(|index| {
            let node = &graph[*index].name;
            node.starts_with("x") || node.starts_with("y")
        })
        .collect();
    let output_indexes: HashSet<NodeIndex> = graph
        .node_indices()
        .filter(|index| graph[*index].name.starts_with("z"))
        .collect();

    for in_index in input_indexes.iter() {
        let in_n: usize = graph[*in_index].name[1..].parse().unwrap();
        let reachable: HashSet<NodeIndex> = Bfs::new(&*graph, *in_index).iter(&*graph).collect();
        let reachable_outputs: HashSet<usize> = output_indexes
            .intersection(&reachable)
            .map(|index| graph[*index].name[1..].parse().unwrap())
            .collect();
        // check that e.g. x03 does not influence z02, but only all outputs from z03 onwards
        assert!(in_n <= *reachable_outputs.iter().max().unwrap());
        let expected: HashSet<usize> = (in_n..output_indexes.len()).collect();
        let diff: HashSet<&usize> = expected.difference(&reachable_outputs).collect();
        // SWAP: z24<->fpq
        if !diff.is_empty() {
            println!("Invalid: Input {in_n} has no path towards Output {diff:?}");
        }
    }

    for index in output_indexes.iter() {
        let n: usize = graph[*index].name[1..].parse().unwrap();
        let in_gate = graph
            .edges_directed(*index, Incoming)
            .next()
            .unwrap()
            .weight();
        if !matches!(in_gate, Gate::XOR) {
            // SWAP: z07<->nqk
            // SWAP: z32<->srn
            println!("Invalid: Output z{n:02} has as gate {in_gate:?}");
        }
    }

    for index in graph.node_indices() {
        if input_indexes.contains(&index) || output_indexes.contains(&index) {
            continue;
        }
        let name = graph[index].name.clone();
        let incoming: Vec<EdgeReference<'_, Gate>> =
            graph.edges_directed(index, Incoming).collect();
        let outgoing: Vec<EdgeReference<'_, Gate>> =
            graph.edges_directed(index, Outgoing).collect();
        // Carry bits (or last z45)
        if outgoing.len() == 1 && matches!(outgoing[0].weight(), Gate::OR) {
            if incoming.len() != 2 || !matches!(incoming[0].weight(), Gate::AND) {
                // SWAP : fgt<->pcp
                println!(
                    "Suspicious carry bit: {name} ({} in, {} out)",
                    incoming.len(),
                    outgoing.len()
                );
            }
        }
    }
    println!("{}", graph.node_indices().len());
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{parser::parse_input, part_one};

    #[test]
    fn part_one_small() -> Result<()> {
        let mut graph = parse_input("small.txt")?;

        let result = part_one(&mut graph);

        assert_eq!(4, result);
        Ok(())
    }

    #[test]
    fn part_one_sample() -> Result<()> {
        let mut graph = parse_input("sample.txt")?;

        let result = part_one(&mut graph);

        assert_eq!(2024, result);
        Ok(())
    }
}
