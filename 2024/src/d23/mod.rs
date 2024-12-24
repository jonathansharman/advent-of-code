use aoc::{graph::Graph, input};

aoc::test::test_part!(test1, part1, 1173);
aoc::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut graph = Graph::new();
	for line in input!().lines() {
		let (left, right) = line.split_once('-').unwrap();
		graph.insert_edge(left, right, 1);
	}
	let mut sets = 0;
	let nodes: Vec<_> = graph.nodes().iter().collect();
	for (i, &n1) in nodes.iter().enumerate() {
		for (j, &n2) in nodes.iter().enumerate().skip(i) {
			for &n3 in nodes.iter().skip(j) {
				if (n1.starts_with('t')
					|| n2.starts_with('t')
					|| n3.starts_with('t'))
					&& graph.weight(n1, n2).is_some()
					&& graph.weight(n1, n3).is_some()
					&& graph.weight(n2, n3).is_some()
				{
					sets += 1;
				}
			}
		}
	}
	sets
}

pub fn part2() -> usize {
	0
}
