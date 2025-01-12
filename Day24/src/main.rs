use petgraph::graph::{DiGraph, NodeIndex};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;
use petgraph::visit::Topo;

fn main() {
    let filename = "input.txt";
    let contents = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let initial_wire_values = parse_initial_values(&lines);
    let gates = parse_gates(&lines);
    let graph = build_circuit_graph(&gates);
    let wire_values = evaluate_circuit_topo(&graph, &initial_wire_values);

    let decimal = wires_to_decimal(&wire_values);
    println!("Decimal value of the output wires: {}", decimal);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Wire(String);

impl Wire {
    fn new(name: &str) -> Self {
        Wire(name.to_string())
    }

    fn name(&self) -> &str {
        &self.0
    }
}



impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Gate {
    op: Op,
    input1: Wire,
    input2: Wire,
    output: Wire,
}

#[derive(Debug, Clone)]
enum NodeData {
    Wire(),
    Gate(Gate),
}

fn parse_initial_values(lines: &[&str]) -> HashMap<Wire, bool> {
    let mut wires = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let wire = Wire::new(parts[0].trim());
            let value = parts[1].trim() == "1";
            wires.insert(wire, value);
        }
    }
    wires
}

fn parse_gates(lines: &[&str]) -> Vec<Gate> {
    let mut gates = Vec::new();
    for line in lines {
        // Example line: "x00 AND y00 -> z00"
        let parts: Vec<&str> = line.split("->").collect();
        if parts.len() != 2 {
            continue; // a wire
        }
        let output = Wire::new(parts[1].trim());
        let gate_parts: Vec<&str> = parts[0].trim().split_whitespace().collect();
        if gate_parts.len() != 3 {
            continue; // Invalid gate format
        }
        let input1 = Wire::new(gate_parts[0]);
        let op = match gate_parts[1] {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => continue, // Unsupported operation
        };
        let input2 = Wire::new(gate_parts[2]);
        gates.push(Gate { op, input1, input2, output });
    }
    gates
}

fn build_circuit_graph(gates: &[Gate]) -> DiGraph<NodeData, ()> {
    let mut graph = DiGraph::<NodeData, ()>::new();
    let mut wire_nodes: HashMap<Wire, NodeIndex> = HashMap::new();

    // Add all unique wires as Wire nodes
    for gate in gates {
        for wire in &[gate.input1.clone(), gate.input2.clone(), gate.output.clone()] {
            if !wire_nodes.contains_key(wire) {
                let node = graph.add_node(NodeData::Wire());
                wire_nodes.insert(wire.clone(), node);
            }
        }
    }

    // Add Gate nodes and connect them to their input and output wires
    for gate in gates {
        let gate_node = graph.add_node(NodeData::Gate(gate.clone()));

        // Connect input wires to the gate
        if let Some(&input1_node) = wire_nodes.get(&gate.input1) {
            graph.add_edge(input1_node, gate_node, ());
        }
        if let Some(&input2_node) = wire_nodes.get(&gate.input2) {
            graph.add_edge(input2_node, gate_node, ());
        }

        // Connect gate to its output wire
        if let Some(&output_node) = wire_nodes.get(&gate.output) {
            graph.add_edge(gate_node, output_node, ());
        }
    }

    graph
}

fn evaluate_circuit_topo(
    graph: &DiGraph<NodeData, ()>,
    initial_wires: &HashMap<Wire, bool>,
) -> HashMap<Wire, bool> {
    let mut wire_values: HashMap<Wire, bool> = initial_wires.clone();

    // Initialize topological sorter
    let mut topo = Topo::new(&graph);

    // Iterate over nodes in topological order
    while let Some(node) = topo.next(&graph) {
        match &graph[node] {
            NodeData::Gate(ref gate) => {
                // Retrieve input wire values
                let input1_val = wire_values.get(&gate.input1);
                let input2_val = wire_values.get(&gate.input2);

                // Proceed only if both inputs have been evaluated
                if let (Some(&val1), Some(&val2)) = (input1_val, input2_val) {
                    // Evaluate gate based on its operation
                    let output_val = match gate.op {
                        Op::And => val1 && val2,
                        Op::Or => val1 || val2,
                        Op::Xor => val1 ^ val2,
                    };

                    // Assign the computed value to the output wire
                    wire_values.insert(gate.output.clone(), output_val);
                }
                // If inputs are not yet available, the topological order should ensure they are processed first
            },
            NodeData::Wire() => {
                // Wires as nodes don't require processing
                continue;
            },
        }
    }

    wire_values
}

fn wires_to_decimal(wire_values: &HashMap<Wire, bool>) -> u64 {
    wire_values
        .keys()
        .filter(|w| w.name().starts_with('z'))
        // Sort wires based on their numeric suffix (e.g., z00, z01, ...)
        .sorted_by_key(|w| {
            // Extract the numeric part after 'z' and parse it as usize
            w.name()[1..].parse::<usize>().unwrap_or(0)
        })
        // Map each wire to its corresponding bit (1 or 0)
        .map(|w| if wire_values[w] { 1u64 } else { 0u64 })
        // Reverse the bits to align with the desired binary order
        .rev()
        // Fold the bits into a single decimal number
        .fold(0u64, |acc, bit| (acc << 1) | bit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_test_input(){
        let filename = "test_input.txt";
        let contents = std::fs::read_to_string(filename).unwrap();
        let lines: Vec<&str> = contents.lines().collect();
        let initial_wire_values = parse_initial_values(&lines);
        let gates = parse_gates(&lines);
        assert_eq!(initial_wire_values.len(), 10);
        assert_eq!(gates.len(), 36);
    }

    #[test]
    fn test_build_circuit_graph(){
        let filename = "test_input.txt";
        let contents = std::fs::read_to_string(filename).unwrap();
        let lines: Vec<&str> = contents.lines().collect();
        let gates = parse_gates(&lines);
        let graph = build_circuit_graph(&gates);
        assert_eq!(graph.node_count(), 82);
        assert_eq!(graph.edge_count(), 108);
    }

    #[test]
    fn test_evaluate_circuit_topo(){
        let filename = "test_input.txt";
        let contents = std::fs::read_to_string(filename).unwrap();
        let lines: Vec<&str> = contents.lines().collect();
        let initial_wire_values = parse_initial_values(&lines);
        let gates = parse_gates(&lines);
        let graph = build_circuit_graph(&gates);
        let wire_values = evaluate_circuit_topo(&graph, &initial_wire_values);

        let decimal = wires_to_decimal(&wire_values);
        assert_eq!(decimal, 2024);

    }
}
