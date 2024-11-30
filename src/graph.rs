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
pub struct Digraph<T: Node> {
	edges: HashMap<T, HashMap<T, usize>>,
}

impl<T: Node> Digraph<T> {
	/// Creates a new empty `Digraph`.
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
	pub fn remove_edge<Q>(&mut self, from: &Q, to: &Q)
	where
		T: Borrow<Q>,
		Q: Hash + Eq + ?Sized,
	{
		if let Some(neighbors) = self.edges.get_mut(from) {
			neighbors.remove(to);
		}
	}

	/// The set of outgoing edges (node → weight) from `from`.
	pub fn edges_from<Q>(&self, from: &Q) -> Option<&HashMap<T, usize>>
	where
		T: Borrow<Q>,
		Q: Hash + Eq + ?Sized,
	{
		self.edges.get(from)
	}

	// TODO: Maybe I should track nodes eagerly to make this O(1), especially
	// since this is used in some other methods.
	/// The set of nodes in the digraph. O(|N|+|E|).
	pub fn get_nodes(&self) -> HashSet<T> {
		let mut nodes = HashSet::new();
		for (node, edges) in self.edges.iter() {
			nodes.insert(node.clone());
			nodes.extend(edges.keys().cloned());
		}
		nodes
	}

	/// Consumes the digraph to produce a list of its strongly connected
	/// components.
	pub fn into_strongly_connected_components(mut self) -> Vec<Self> {
		let mut components = Vec::new();
		while !self.edges.is_empty() {
			let mut component = Digraph::new();
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

	/// A Graphviz representation of the digraph.
	pub fn graphviz(&self) -> String
	where
		T: std::fmt::Display + Ord,
	{
		let mut output = String::new();
		output += "digraph {\n";
		self.edges
			.iter()
			.flat_map(|(node, edges)| {
				edges
					.iter()
					.map(move |(neighbor, weight)| (node, neighbor, weight))
			})
			.sorted()
			.for_each(|(a, b, w)| {
				output += &format!(
					"\t\"{a}\" -> \"{b}\" \
					[label=\"{w}\" \
					tooltip=\"{a}→{b}: {w}\"]\n"
				);
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
			if let Some(edges) = self.edges_from(&node) {
				for (neighbor, weight) in edges {
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
		}
		None
	}

	/// The shortest distance from each node to each other node. Uses the
	/// Floyd-Warshall algorithm.
	pub fn shortest_distances(&self) -> HashMap<T, HashMap<T, usize>> {
		let nodes = self.get_nodes();
		let mut distances: HashMap<T, HashMap<T, usize>> = HashMap::new();
		for node in &nodes {
			let node_distances = distances.entry(node.clone()).or_default();
			node_distances.insert(node.clone(), 0);
			if let Some(edges) = self.edges_from(node) {
				for (neighbor, &weight) in edges {
					node_distances.insert(neighbor.clone(), weight);
				}
			}
		}
		for i in &nodes {
			for j in &nodes {
				for k in &nodes {
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

impl<T: Node> std::fmt::Debug for Digraph<T>
where
	T: std::fmt::Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Digraph")
			.field("edges", &self.edges)
			.finish()
	}
}

/// An undirected graph. Implemented using [`Digraph`].
#[derive(Clone, Default)]
pub struct Graph<T: Node>(Digraph<T>);

impl<T: Node> Graph<T> {
	/// Creates a new empty `Graph`.
	pub fn new() -> Self {
		Self(Digraph::new())
	}

	/// Adds an edge between nodes `a` and `b` with the given `weight`.
	pub fn insert_edge(&mut self, a: T, b: T, weight: usize) {
		self.0.insert_edge(a.clone(), b.clone(), weight);
		self.0.insert_edge(b, a, weight);
	}

	/// Removes any edge between nodes `a` and `b`.
	pub fn remove_edge<Q>(&mut self, a: &Q, b: &Q)
	where
		T: Borrow<Q>,
		Q: Hash + Eq + ?Sized,
	{
		self.0.remove_edge(a, b);
		self.0.remove_edge(b, a);
	}

	/// The set of edges (node → weight) from `from`.
	pub fn edges_from<Q>(&self, from: &Q) -> Option<&HashMap<T, usize>>
	where
		T: Borrow<Q>,
		Q: Hash + Eq + ?Sized,
	{
		self.0.edges_from(from)
	}

	/// The set of nodes in the graph. O(|N|+|E|).
	pub fn get_nodes(&self) -> HashSet<T> {
		self.0.get_nodes()
	}

	/// Consumes the graph to produce a list of its connected components.
	pub fn into_connected_components(self) -> Vec<Self> {
		self.0
			.into_strongly_connected_components()
			.into_iter()
			.map(|g| Self(g))
			.collect()
	}

	/// A Graphviz representation of the graph.
	pub fn graphviz(&self) -> String
	where
		T: std::fmt::Display + Ord,
	{
		let mut output = String::new();
		output += "graph {\n";
		let mut nodes = Vec::from_iter(self.get_nodes());
		nodes.sort();
		let mut unique_edges = HashSet::new();
		for node in &nodes {
			if let Some(edges) = self.edges_from(node) {
				for (neighbor, weight) in edges {
					if !unique_edges.contains(&(neighbor, node, weight)) {
						unique_edges.insert((node, neighbor, weight));
					}
				}
			}
		}
		unique_edges.into_iter().sorted().for_each(|(a, b, w)| {
			output += &format!(
				"\t\"{a}\" -- \"{b}\" \
				[label=\"{w}\" \
				tooltip=\"{a}-{b}: {w}\"]\n"
			);
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
		self.0.shortest_distance(start, pred)
	}

	/// The shortest distance from each node to each other node. Uses the
	/// Floyd-Warshall algorithm.
	pub fn shortest_distances(&self) -> HashMap<T, HashMap<T, usize>> {
		self.0.shortest_distances()
	}
}

impl<T: Node + Ord> std::fmt::Debug for Graph<T>
where
	T: std::fmt::Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("Graph").field(&self.0).finish()
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

	fn new_test_digraph() -> Digraph<&'static str> {
		let mut graph = Digraph::new();
		graph.insert_edge("start", "a", 1);
		graph.insert_edge("start", "shortcut", 2);
		graph.insert_edge("a", "b", 2);
		graph.insert_edge("b", "c", 1);
		graph.insert_edge("c", "goal", 1);
		graph.insert_edge("shortcut", "goal", 1);
		graph
	}

	fn new_test_graph() -> Graph<&'static str> {
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
	fn digraph_shortest_distance() {
		let g = new_test_digraph();
		let d = g.shortest_distance("start", |&n| n == "goal");
		assert_eq!(d, Some(3));
		let d = g.shortest_distance("goal", |&n| n == "start");
		assert!(d.is_none());
	}

	#[test]
	fn graph_shortest_distance() {
		let g = new_test_graph();
		let d = g.shortest_distance("start", |&n| n == "goal");
		assert_eq!(d, Some(3));
		let d = g.shortest_distance("goal", |&n| n == "start");
		assert_eq!(d, Some(3));
	}

	#[test]
	fn digraph_shortest_distances() {
		let d = new_test_digraph().shortest_distances();
		assert_eq!(d["start"]["start"], 0);
		assert_eq!(d["start"]["goal"], 3);
		assert_eq!(d["a"]["c"], 3);
		assert_eq!(d["goal"]["goal"], 0);
		assert!(!d["goal"].contains_key("start"));
	}

	#[test]
	fn graph_shortest_distances() {
		let d = new_test_graph().shortest_distances();
		assert_eq!(d["start"]["start"], 0);
		assert_eq!(d["start"]["goal"], 3);
		assert_eq!(d["a"]["c"], 3);
		assert_eq!(d["goal"]["goal"], 0);
		assert_eq!(d["goal"]["start"], 3);
	}

	#[test]
	fn digraph_graphviz() {
		let graphviz = new_test_digraph().graphviz();
		assert_eq!(
			graphviz,
			r#"digraph {
	"a" -> "b" [label="2" tooltip="a→b: 2"]
	"b" -> "c" [label="1" tooltip="b→c: 1"]
	"c" -> "goal" [label="1" tooltip="c→goal: 1"]
	"shortcut" -> "goal" [label="1" tooltip="shortcut→goal: 1"]
	"start" -> "a" [label="1" tooltip="start→a: 1"]
	"start" -> "shortcut" [label="2" tooltip="start→shortcut: 2"]
}
"#,
		);
	}

	#[test]
	fn graph_graphviz() {
		let graphviz = new_test_graph().graphviz();
		assert_eq!(
			graphviz,
			r#"graph {
	"a" -- "b" [label="2" tooltip="a-b: 2"]
	"a" -- "start" [label="1" tooltip="a-start: 1"]
	"b" -- "c" [label="1" tooltip="b-c: 1"]
	"c" -- "goal" [label="1" tooltip="c-goal: 1"]
	"goal" -- "shortcut" [label="1" tooltip="goal-shortcut: 1"]
	"shortcut" -- "start" [label="2" tooltip="shortcut-start: 2"]
}
"#,
		);
	}
}
