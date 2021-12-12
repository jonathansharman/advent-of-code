use crate::io::read_lines;

use itertools::Itertools;

use std::collections::{HashMap, HashSet};

crate::test::test_part!(test1, part1, 4707);
crate::test::test_part!(test2, part2, 130493);

pub fn part1() -> i64 {
	count_paths_1(&read_graph(), "start", "end")
}

pub fn part2() -> i64 {
	count_paths_2(&read_graph(), HashMap::new(), false, "start", "end")
}

type Graph = HashMap<String, HashSet<String>>;

fn read_graph() -> Graph {
	let mut graph = HashMap::new();
	for line in read_lines("input/2021/12.txt") {
		let split = line.split('-').collect_vec();
		let (a, b) = (split[0], split[1]);
		graph
			.entry(a.to_owned())
			.or_insert_with(HashSet::new)
			.insert(b.to_owned());
		graph
			.entry(b.to_owned())
			.or_insert_with(HashSet::new)
			.insert(a.to_owned());
	}
	graph
}

fn count_paths_1(graph: &Graph, from: &str, to: &str) -> i64 {
	if from == to {
		return 1;
	}
	if from.chars().all(char::is_uppercase) {
		graph[from]
			.iter()
			.map(|neighbor| count_paths_1(graph, neighbor, to))
			.sum()
	} else {
		match graph.get(from) {
			Some(neighbors) => {
				let mut graph = graph.clone();
				graph.remove(from);
				neighbors
					.iter()
					.map(|neighbor| count_paths_1(&graph, neighbor, to))
					.sum()
			}
			None => 0,
		}
	}
}

fn count_paths_2(
	graph: &Graph,
	mut visit_map: HashMap<String, u32>,
	double_used: bool,
	from: &str,
	to: &str,
) -> i64 {
	if from == to {
		return 1;
	}
	let visits = visit_map.entry(from.into()).or_insert(0);
	*visits += 1;
	if !from.chars().all(char::is_uppercase) {
		if *visits == 1 {
			return graph[from]
				.iter()
				.map(|neighbor| count_paths_2(graph, visit_map.clone(), double_used, neighbor, to))
				.sum();
		}
		if *visits == 2 && !double_used && from != "start" {
			return graph[from]
				.iter()
				.map(|neighbor| count_paths_2(graph, visit_map.clone(), true, neighbor, to))
				.sum();
		}
		return 0;
	}
	graph[from]
		.iter()
		.map(|neighbor| count_paths_2(graph, visit_map.clone(), double_used, neighbor, to))
		.sum()
}
