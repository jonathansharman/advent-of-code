use std::{
	borrow::Borrow,
	collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
	hash::Hash,
};

use itertools::Itertools;

use crate::neighbors;

pub trait Node: Clone + Eq + std::hash::Hash {}

impl<T> Node for T where T: Clone + Eq + std::hash::Hash {}

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
#[derive(Clone, Default)]
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

	/// Adds an edge from node `from` to node `to` with the given `weight`.
	pub fn insert_edge(&mut self, from: T, to: T, weight: usize) {
		self.edges.entry(from).or_default().insert(to, weight);
	}

	/// Removes any edge from node `from` to node `to`.
	pub fn remove_edge<Q: ?Sized>(&mut self, from: &Q, to: &Q)
	where
		T: Borrow<Q>,
		Q: Hash + Eq,
	{
		if let Some(neighbors) = self.edges.get_mut(from) {
			neighbors.remove(to);
		}
	}

	/// The set of outgoing edges (node → weight) from `from`.
	pub fn edges_from<Q: ?Sized>(&self, from: &Q) -> &HashMap<T, usize>
	where
		T: Borrow<Q>,
		Q: Hash + Eq,
	{
		&self.edges[from]
	}

	/// The number of nodes in this graph. O(|N|+|E|).
	pub fn node_count(&self) -> usize {
		let mut nodes = HashSet::new();
		for (node, edges) in self.edges.iter() {
			nodes.insert(node.clone());
			nodes.extend(edges.keys().cloned());
		}
		nodes.len()
	}

	/// Consumes the graph to produce a list of its strongly connected
	/// components.
	pub fn into_strongly_connected_components(mut self) -> Vec<Self> {
		let mut components = Vec::new();
		while !self.edges.is_empty() {
			let mut component = Graph::new();
			let mut queue = vec![self.edges.keys().next().unwrap().clone()];
			while let Some(node) = queue.pop() {
				if let Some(edges) = self.edges.remove(&node) {
					for (neighbor, weight) in edges {
						component.insert_edge(
							node.clone(),
							neighbor.clone(),
							weight,
						);
						queue.push(neighbor);
					}
				}
			}
			components.push(component);
		}
		components
	}

	/// A Graphviz representation of the graph.
	pub fn graphviz(&self) -> String
	where
		T: std::fmt::Display,
	{
		let mut output = String::new();
		output += "digraph {\n";
		for (a, edges) in &self.edges {
			for (b, weight) in edges {
				output += 
					&format!("\t\"{a}\" -> \"{b}\" [label=\"{weight}\" tooltip=\"{a}→{b}: {weight}\"]\n");
			}
		}
		output += "}\n";
		output
	}

	/// A Graphviz representation of the graph, treating the graph as
	/// undirected. The results are only meaningful if all the edges (including
	/// weights) are symmetric.
	pub fn graphviz_undirected(&self) -> String
	where
		T: std::fmt::Display + Ord,
	{
		let mut output = String::new();
		output += "graph {\n";
		self.edges
			.iter()
			.fold(HashMap::new(), |mut acc, (node, neighbors)| {
				for (neighbor, weight) in neighbors {
					let key = (node.min(neighbor), node.max(neighbor));
					acc.insert(key, weight);
				}
				acc
			})
			.into_iter()
			.sorted()
			.for_each(|((a, b), weight)| {
				output +=
					&format!("\t\"{a}\" -- \"{b}\" [label=\"{weight}\" tooltip=\"{a}-{b}: {weight}\"]\n");
			});
		output += "}\n";
		output
	}

	/// The shortest distance from `start` to a node that satisfies `pred` or
	/// `None` if no such node is reachable. Uses Dijkstra's algorithm.
	pub fn shortest_distance<F>(&self, start: T, pred: F) -> Option<usize>
	where
		F: Fn(&T) -> bool,
	{
		let mut distances: HashMap<T, usize> = HashMap::new();
		let mut queue: BinaryHeap<State<T>> = BinaryHeap::new();
		distances.insert(start.clone(), 0);
		queue.push(State {
			distance: 0,
			node: start,
		});
		while let Some(State { distance, node }) = queue.pop() {
			if pred(&node) {
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
						node: neighbor.clone(),
					});
					distances.insert(neighbor.clone(), candidate);
				}
			}
		}
		None
	}

	/// The shortest distance from each node to each other node. Uses the
	/// Floyd-Warshall algorithm.
	pub fn shortest_distances(&self) -> HashMap<T, HashMap<T, usize>> {
		let mut distances: HashMap<T, HashMap<T, usize>> = HashMap::new();
		for (node, neighbors) in self.edges.iter() {
			let node_distances = distances.entry(node.clone()).or_default();
			node_distances.insert(node.clone(), 0);
			for (neighbor, &weight) in neighbors {
				node_distances.insert(neighbor.clone(), weight);
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
						match distances.get_mut(i).unwrap().entry(j.clone()) {
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

impl<T: Node> std::fmt::Debug for Graph<T>
where
	T: std::fmt::Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Graph").field("edges", &self.edges).finish()
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
		let d = graph.shortest_distance("start", |&n| n == "goal");
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
