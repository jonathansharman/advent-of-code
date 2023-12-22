use std::collections::{hash_map::Entry, BinaryHeap, HashMap};

use crate::neighbors;

pub trait Node: Copy + Eq + std::hash::Hash {}

impl<T> Node for T where T: Copy + Eq + std::hash::Hash {}

#[derive(PartialEq, Eq)]
struct State<T: Node> {
	distance: usize,
	node: T,
}

impl<T: Node> Ord for State<T> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		other.distance.cmp(&self.distance)
	}
}

impl<T: Node> PartialOrd for State<T> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

/// A directed graph.
#[derive(Default)]
pub struct Graph<T: Node> {
	edges: HashMap<T, HashMap<T, usize>>,
}

impl<T: Node> Graph<T> {
	/// Creates a new empty `Graph`.
	pub fn new() -> Self {
		Self {
			edges: HashMap::new(),
		}
	}

	/// Inserts the given `node` with an empty adjency set, if the node doesn't
	/// already exist.
	pub fn insert_node(&mut self, node: T) {
		self.edges.entry(node).or_default();
	}

	/// Calls [`insert_node`] with each item of `nodes`.
	pub fn extend_nodes<I: IntoIterator<Item = T>>(&mut self, nodes: I) {
		for node in nodes {
			self.insert_node(node);
		}
	}

	/// Adds an edge from node `from` to node `to` with the given `weight`.
	pub fn insert_edge(&mut self, from: T, to: T, weight: usize) {
		self.edges.get_mut(&from).unwrap().insert(to, weight);
	}

	/// The shortest distance from `start` to a node that satisfies `pred` or
	/// `None` if no such node is reachable. Uses Dijkstra's algorithm.
	pub fn shortest_distance<F>(&self, start: T, pred: F) -> Option<usize>
	where
		F: Fn(T) -> bool,
	{
		let mut distances: HashMap<T, usize> = HashMap::new();
		let mut queue: BinaryHeap<State<T>> = BinaryHeap::new();
		distances.insert(start, 0);
		queue.push(State {
			distance: 0,
			node: start,
		});
		while let Some(State { distance, node }) = queue.pop() {
			if pred(node) {
				return Some(distance);
			}
			if distance > distances[&node] {
				// We've already reached this node by a shorter path.
				continue;
			}
			for (neighbor, weight) in self.edges[&node].iter() {
				let candidate = distance + weight;
				let d = distances.get(neighbor);
				if d.map_or(true, |d| candidate < *d) {
					queue.push(State {
						distance: candidate,
						node: *neighbor,
					});
					distances.insert(*neighbor, candidate);
				}
			}
		}
		None
	}

	/// The shortest distance from each node to each other node. Uses the
	/// Floyd-Warshall algorithm.
	pub fn shortest_distances(&self) -> HashMap<T, HashMap<T, usize>> {
		let mut distances: HashMap<T, HashMap<T, usize>> = HashMap::new();
		for (&node, neighbors) in self.edges.iter() {
			let node_distances = distances.entry(node).or_default();
			node_distances.insert(node, 0);
			for (&neighbor, &weight) in neighbors {
				node_distances.insert(neighbor, weight);
			}
		}
		let mut count = 0;
		for i in self.edges.keys() {
			count += 1;
			println!("{count}");
			for j in self.edges.keys() {
				for k in self.edges.keys() {
					if let Some(candidate) = distances
						.get(i)
						.and_then(|d| d.get(k))
						.and_then(|left| {
							distances
								.get(k)
								.and_then(|d| d.get(j))
								.map(|right| left + right)
						}) {
						match distances.get_mut(i).unwrap().entry(*j) {
							Entry::Occupied(mut entry) => {
								entry.insert(*entry.get().min(&candidate));
							}
							Entry::Vacant(entry) => {
								entry.insert(candidate);
							}
						}
					}
				}
			}
		}
		distances
	}
}

/// Creates a `Graph<(usize, usize)>` from a grid of open/closed cells. The
/// nodes will be the row-column coordinates of the open cells, and each node
/// will have weight-1 edges to its open four-directional neighbors.
pub fn from_bool_grid(grid: &[Vec<bool>]) -> Graph<(usize, usize)> {
	let mut graph = Graph::new();
	for (i, row) in grid.iter().enumerate() {
		for (j, open) in row.iter().enumerate() {
			let node = (i, j);
			if !open {
				continue;
			}
			graph.insert_node(node);
			for neighbor in neighbors::four(grid.len(), grid[0].len(), i, j) {
				if grid[neighbor.0][neighbor.1] {
					graph.insert_edge(node, neighbor, 1);
				}
			}
		}
	}
	graph
}

#[cfg(test)]
mod tests {
	use super::*;

	fn get_test_graph() -> Graph<&'static str> {
		let mut graph = Graph::new();
		graph.extend_nodes(["start", "a", "b", "c", "shortcut", "goal"]);
		graph.insert_edge("start", "a", 1);
		graph.insert_edge("start", "shortcut", 2);
		graph.insert_edge("a", "b", 2);
		graph.insert_edge("b", "c", 1);
		graph.insert_edge("c", "goal", 1);
		graph.insert_edge("shortcut", "goal", 1);
		graph
	}

	#[test]
	fn shortest_distance() {
		let graph = get_test_graph();
		let d = graph.shortest_distance("start", |n| n == "goal");
		assert_eq!(d, Some(3));
	}

	#[test]
	fn shortest_distances() {
		let graph = get_test_graph();
		let d = graph.shortest_distances();
		assert_eq!(d["start"]["start"], 0);
		assert_eq!(d["start"]["goal"], 3);
		assert_eq!(d["a"]["c"], 3);
		assert_eq!(d["goal"]["goal"], 0);
	}
}
