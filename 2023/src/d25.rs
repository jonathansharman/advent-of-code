use crate::{graph::Graph, io::read_lines};

aoc::test::test_part!(test1, part1, 562772);

const INPUT: &str = include_str!("input/25.txt");

pub fn part1() -> usize {
	let mut graph = Graph::new();
	for line in INPUT.lines() {
		let (node, neighbors) = line.split_once(": ").unwrap();
		for neighbor in neighbors.split_whitespace() {
			graph.insert_edge(node.to_owned(), neighbor.to_owned(), 1);
		}
	}
	// TODO: Find programmatically (instead of by outputting with Graphviz and
	// eyeballing it 😅). Look into the Stoer-Wagner algorithm.
	graph.remove_edge("zkv", "zxb");
	graph.remove_edge("mtl", "pgl");
	graph.remove_edge("lkf", "scf");
	graph
		.into_connected_components()
		.into_iter()
		.map(|g| g.get_nodes().len())
		.product()
}
