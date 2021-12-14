use crate::io::read_lines;

use itertools::Itertools;

use std::collections::{HashMap, HashSet};

crate::test::test_part!(test1, part1, 4707);
crate::test::test_part!(test2, part2, 130493);

pub fn part1() -> i64 {
	count_paths(&read_graph(), &HashMap::new(), true, "start")
}

pub fn part2() -> i64 {
	count_paths(&read_graph(), &HashMap::new(), false, "start")
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

fn count_paths(
	graph: &Graph,
	visit_map: &HashMap<String, u32>,
	double_used: bool,
	from: &str,
) -> i64 {
	if from == "end" {
		return 1;
	}
	let mut visit_map = visit_map.clone();
	let visits = visit_map.entry(from.into()).or_insert(0);
	*visits += 1;
	if from.chars().all(char::is_uppercase) || *visits == 1 {
		graph[from]
			.iter()
			.map(|neighbor| count_paths(graph, &visit_map, double_used, neighbor))
			.sum()
	} else if *visits == 2 && !double_used && from != "start" {
		graph[from]
			.iter()
			.map(|neighbor| count_paths(graph, &visit_map, true, neighbor))
			.sum()
	} else {
		0
	}
}
