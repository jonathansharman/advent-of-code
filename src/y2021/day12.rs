use crate::{io::read_lines, solution::Solution};

use itertools::Itertools;

use std::collections::{HashMap, HashSet};

pub struct Day12;

impl Solution for Day12 {
	fn year(&self) -> u32 {
		2021
	}

	fn day(&self) -> u32 {
		12
	}

	fn part1(&self) -> i64 {
		count_paths(&read_graph(), "start", "end")
	}

	fn part2(&self) -> i64 {
		count_paths(&read_graph(), "start", "end")
	}
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

fn count_paths(graph: &Graph, from: &str, to: &str) -> i64 {
	if from == to {
		return 1;
	}
	if from.chars().all(char::is_uppercase) {
		graph[from]
			.iter()
			.map(|neighbor| count_paths(graph, neighbor, to))
			.sum()
	} else {
		match graph.get(from) {
			Some(neighbors) => {
				let mut graph = graph.clone();
				graph.remove(from);
				neighbors
					.iter()
					.map(|neighbor| count_paths(&graph, neighbor, to))
					.sum()
			}
			None => 0,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() {
		assert_eq!(4707, Day12.part1());
	}

	#[test]
	fn part2() {
		assert_eq!(0, Day12.part2());
	}
}
