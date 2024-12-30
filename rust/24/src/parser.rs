use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use anyhow::Result;
use petgraph::graph::{DiGraph, NodeIndex};

use crate::gates::Gate;

pub type CircuitGraph = DiGraph<Node, Gate>;

pub fn parse_input(path: &str) -> Result<CircuitGraph> {
    let s = read_to_string(path)?;
    let initial_assignment: HashMap<String, bool> = s
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            (
                parts.get(0).unwrap().to_string(),
                parts.get(1).unwrap().parse::<u8>().unwrap() != 0,
            )
        })
        .collect();
    let edges: Vec<(String, Gate, String)> = s
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .flat_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert_eq!(5, parts.len());
            let in_1 = parts[0].to_string();
            let gate = Gate::from(parts[1]).expect("Could not parse gate");
            let in_2 = parts[2].to_string();
            let out = parts[4].to_string();
            [(in_1, gate.clone(), out.clone()), (in_2, gate, out)]
        })
        .collect();

    let mut graph = CircuitGraph::new();
    let mut name_to_index: HashMap<&String, NodeIndex> = HashMap::new();
    let nodes: HashSet<&String> = edges.iter().flat_map(|(from, _, to)| [from, to]).collect();
    for name in nodes {
        let value = initial_assignment.get(name).map(|b| *b);
        let node = Node {
            name: name.clone(),
            value,
        };
        let index = graph.add_node(node);
        name_to_index.insert(name, index);
    }
    for (from, gate, to) in edges.iter() {
        let from_index = name_to_index[from];
        let to_index = name_to_index[to];
        graph.add_edge(from_index, to_index, gate.clone());
    }

    Ok(graph)
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub value: Option<bool>,
}
