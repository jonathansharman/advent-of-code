use crate::{graph::Graph, io::read_lines};

crate::test::test_part!(test1, part1, 562772);

pub fn part1() -> usize {
	let mut graph = Graph::new();
	for line in read_lines("input/2023/25.txt") {
		let (node, neighbors) = line.split_once(": ").unwrap();
		for neighbor in neighbors.split_whitespace() {
			graph.insert_edge(node.to_owned(), neighbor.to_owned(), 1);
			graph.insert_edge(neighbor.to_owned(), node.to_owned(), 1);
		}
	}
	// TODO: Find programmatically (instead of by outputting with Graphviz and
	// eyeballing it 😅).
	graph.remove_edge("zkv", "zxb");
	graph.remove_edge("zxb", "zkv");
	graph.remove_edge("mtl", "pgl");
	graph.remove_edge("pgl", "mtl");
	graph.remove_edge("lkf", "scf");
	graph.remove_edge("scf", "lkf");
	graph
		.into_strongly_connected_components()
		.into_iter()
		.map(|g| g.node_count())
		.product()
}
