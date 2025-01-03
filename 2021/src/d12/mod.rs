use aoc::input;
use itertools::Itertools;

use std::collections::{HashMap, HashSet};

aoc::test::test_part!(test1, part1, 4707);
aoc::test::test_part!(test2, part2, 130493);

pub fn part1() -> i64 {
	read_graph().count_paths_from_start(false)
}

pub fn part2() -> i64 {
	read_graph().count_paths_from_start(true)
}

struct NamedNode {
	idx: usize,
	big: bool,
	neighbors: HashSet<String>,
}

fn read_graph() -> Graph {
	let mut named_nodes = HashMap::new();

	for line in input!().lines() {
		let mut add_edge = |a: &str, b: &str| {
			let n = named_nodes.len();
			named_nodes
				.entry(a.to_owned())
				.or_insert_with(|| NamedNode {
					idx: n,
					big: a.chars().all(char::is_uppercase),
					neighbors: HashSet::new(),
				})
				.neighbors
				.insert(b.to_owned());
		};

		let (a, b) = line.split('-').collect_tuple().unwrap();
		add_edge(a, b);
		add_edge(b, a);
	}

	let mut nodes = vec![Node::default(); named_nodes.len()];
	for string_node in named_nodes.values() {
		nodes[string_node.idx] = Node {
			big: string_node.big,
			neighbors: string_node
				.neighbors
				.iter()
				.map(|name| named_nodes[name].idx)
				.collect(),
		};
	}

	Graph {
		start_idx: named_nodes["start"].idx,
		end_idx: named_nodes["end"].idx,
		nodes,
	}
}

#[derive(Clone, Default)]
struct Node {
	big: bool,
	neighbors: Vec<usize>,
}

struct Graph {
	start_idx: usize,
	end_idx: usize,
	nodes: Vec<Node>,
}

impl Graph {
	fn count_paths_from_start(&self, allow_double_crossing: bool) -> i64 {
		self.count_paths(
			&vec![0; self.nodes.len()],
			!allow_double_crossing,
			self.start_idx,
		)
	}

	fn count_paths(
		&self,
		visits: &[u32],
		double_used: bool,
		from: usize,
	) -> i64 {
		if from == self.end_idx {
			return 1;
		}
		let mut visits = Vec::from(visits);
		visits[from] += 1;
		if self.nodes[from].big || visits[from] == 1 {
			self.nodes[from]
				.neighbors
				.iter()
				.map(|neighbor| {
					self.count_paths(&visits, double_used, *neighbor)
				})
				.sum()
		} else if visits[from] == 2 && !double_used && from != self.start_idx {
			self.nodes[from]
				.neighbors
				.iter()
				.map(|neighbor| self.count_paths(&visits, true, *neighbor))
				.sum()
		} else {
			0
		}
	}
}
