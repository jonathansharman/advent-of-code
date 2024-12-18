use std::{
	borrow::Borrow,
	collections::{BinaryHeap, HashMap, HashSet},
	hash::Hash,
};

use itertools::Itertools;

use crate::grid::{Grid, Point};

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
	nodes: HashSet<T>,
	edges: HashMap<T, HashMap<T, usize>>,
}

/// The results of a call to Dijkstra's algorithm for a particular start node.
pub struct DijkstraResults<T: Node> {
	/// A (node → parent nodes) map, which steps from each node through all
	/// shortest paths to the start node.
	parents: HashMap<T, HashSet<T>>,
	/// A (node → distance) map, representing the length of the shortest path
	/// from the start node to each other node.
	distances: HashMap<T, usize>,
}

impl<T: Node> DijkstraResults<T> {
	/// The shortest distance from the start node to any node that satisfies
	/// `pred` or `None` if no such node is reachable.
	pub fn shortest_distance<F>(&self, pred: F) -> Option<usize>
	where
		F: Fn(&T) -> bool,
	{
		self.distances
			.iter()
			.filter_map(|(node, d)| pred(node).then_some(d))
			.min()
			.copied()
	}

	/// The shortest distance from the start node to `end`.
	pub fn distance<Q>(&self, end: &Q) -> Option<usize>
	where
		T: Borrow<Q>,
		Q: Hash + Eq + ?Sized,
	{
		self.distances.get(end).copied()
	}

	/// A digraph from `end` back through all shortest paths to the start node.
	pub fn backtrace(&self, end: T) -> Digraph<T> {
		let mut digraph = Digraph::new();
		let mut queue = vec![end];
		while let Some(node) = queue.pop() {
			let Some(parents) = self.parents.get(&node) else {
				continue;
			};
			for parent in parents {
				digraph.insert_edge(node.clone(), parent.clone(), 1);
				queue.push(parent.clone());
			}
		}
		digraph
	}
}

impl<T: Node> Digraph<T> {
	/// Creates a new empty `Digraph`.
	pub fn new() -> Self {
		Self {
			nodes: HashSet::new(),
			edges: HashMap::new(),
		}
	}

	/// Adds an edge from node `from` to node `to` with the given `weight` or
	/// updates the weight if the edge already exists. Returns the previous
	/// weight, if there was one. The nodes will be created if they don't
	/// already exist.
	pub fn insert_edge(
		&mut self,
		from: T,
		to: T,
		weight: usize,
	) -> Option<usize> {
		self.nodes.insert(from.clone());
		self.nodes.insert(to.clone());
		self.edges.entry(from).or_default().insert(to, weight)
	}

	/// Removes the edge from node `from` to node `to`, if there is one. Note
	/// that the nodes themselves will not be removed, even if they no longer
	/// have any edges.
	pub fn remove_edge<Q>(&mut self, from: &Q, to: &Q)
	where
		T: Borrow<Q>,
		Q: Hash + Eq + ?Sized,
	{
		if let Some(neighbors) = self.edges.get_mut(from) {
			neighbors.remove(to);
		}
	}

	/// The weight of the edge from node `from` to node `to`, if any.
	pub fn weight<Q>(&self, from: &Q, to: &Q) -> Option<usize>
	where
		T: Borrow<Q>,
		Q: Hash + Eq + ?Sized,
	{
		self.edges
			.get(from)
			.and_then(|edges| edges.get(to).copied())
	}

	/// The set of outgoing edges (node → weight) from `from`.
	pub fn edges_from<Q>(&self, from: &Q) -> Option<&HashMap<T, usize>>
	where
		T: Borrow<Q>,
		Q: Hash + Eq + ?Sized,
	{
		self.edges.get(from)
	}

	/// Converts the digraph into its set of nodes.
	pub fn into_nodes(self) -> HashSet<T> {
		self.nodes
	}

	/// The set of nodes in the digraph.
	pub fn nodes(&self) -> &HashSet<T> {
		&self.nodes
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

	/// The shortest distance from `start` to any node that satisfies `pred` or
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

	/// Uses Dijkstra's algorithm to compute shortest path info from the `start`
	/// node to all other nodes.
	pub fn one_to_all_shortest_paths(&self, start: T) -> DijkstraResults<T> {
		let mut parents: HashMap<T, HashSet<T>> = HashMap::new();
		let mut distances = HashMap::from([(start.clone(), 0)]);

		let mut queue: BinaryHeap<State<T>> = BinaryHeap::from([State {
			distance: 0,
			node: start,
		}]);
		while let Some(State { distance, node }) = queue.pop() {
			let Some(edges) = self.edges_from(&node) else {
				continue;
			};
			for (neighbor, weight) in edges {
				let candidate = distance + weight;
				let d = distances.get(neighbor);
				if d.map_or(true, |&d| candidate < d) {
					queue.push(State {
						distance: candidate,
						node: neighbor.clone(),
					});
					*parents.entry(neighbor.clone()).or_default() =
						[node.clone()].into();
					distances.insert(neighbor.clone(), candidate);
				} else if d.map_or(true, |&d| candidate == d) {
					queue.push(State {
						distance: candidate,
						node: neighbor.clone(),
					});
					parents
						.entry(neighbor.clone())
						.or_default()
						.insert(node.clone());
					distances.insert(neighbor.clone(), candidate);
				}
			}
		}
		DijkstraResults { parents, distances }
	}

	/// The shortest distance from each node to each other node. Uses the
	/// Floyd-Warshall algorithm. The returned digraph will contain an edge from
	/// each node to each other reachable node, with a weight equal to the
	/// shortest distance between them.
	pub fn all_to_all_shortest_distances(&self) -> Digraph<T> {
		let mut distances = self.clone();
		for node in &self.nodes {
			distances.insert_edge(node.clone(), node.clone(), 0);
		}
		for i in &self.nodes {
			for j in &self.nodes {
				for k in &self.nodes {
					if let Some(candidate) = distances
						.edges_from(i)
						.and_then(|d| d.get(k))
						.and_then(|left| {
							distances
								.edges_from(k)
								.and_then(|d| d.get(j))
								.map(|right| left + right)
						}) {
						let weight = distances
							.weight(i, j)
							.map_or(candidate, |current| {
								current.min(candidate)
							});
						distances.insert_edge(i.clone(), j.clone(), weight);
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

	/// Adds an edge between nodes `a` and `b` with the given `weight` or
	/// updates the weight if the edge already exists. Returns the previous
	/// weight, if there was one.
	pub fn insert_edge(&mut self, a: T, b: T, weight: usize) -> Option<usize> {
		self.0.insert_edge(a.clone(), b.clone(), weight);
		self.0.insert_edge(b, a, weight)
	}

	/// Removes the edge between nodes `a` and `b`, if there is one.
	pub fn remove_edge<Q>(&mut self, a: &Q, b: &Q)
	where
		T: Borrow<Q>,
		Q: Hash + Eq + ?Sized,
	{
		self.0.remove_edge(a, b);
		self.0.remove_edge(b, a);
	}

	/// The weight of the edge from node `from` to node `to`, if any.
	pub fn weight<Q>(&self, from: &Q, to: &Q) -> Option<usize>
	where
		T: Borrow<Q>,
		Q: Hash + Eq + ?Sized,
	{
		self.0.weight(from, to)
	}

	/// The set of edges (node → weight) from `from`.
	pub fn edges_from<Q>(&self, from: &Q) -> Option<&HashMap<T, usize>>
	where
		T: Borrow<Q>,
		Q: Hash + Eq + ?Sized,
	{
		self.0.edges_from(from)
	}

	/// Converts the graph into its set of nodes.
	pub fn into_nodes(self) -> HashSet<T> {
		self.0.into_nodes()
	}

	/// The set of nodes in the graph.
	pub fn nodes(&self) -> &HashSet<T> {
		self.0.nodes()
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
		let mut nodes = Vec::from_iter(self.nodes());
		nodes.sort();
		let mut unique_edges = HashSet::new();
		for node in nodes {
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

	/// Uses Dijkstra's algorithm to compute shortest path info from the `start`
	/// node to an arbitrary node that satisfies `pred`. If no such path exists,
	/// this returns `None`.
	pub fn one_to_all_shortest_paths<F>(&self, start: T) -> DijkstraResults<T>
	where
		F: Fn(&T) -> bool,
	{
		self.0.one_to_all_shortest_paths(start)
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
	/// Floyd-Warshall algorithm. The returned digraph will contain an edge from
	/// each node to each other reachable node, with a weight equal to the
	/// shortest distance between them.
	pub fn all_to_all_shortest_distances(&self) -> Digraph<T> {
		self.0.all_to_all_shortest_distances()
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

/// Creates a `Digraph<Point>` from a grid of open/closed cells. The nodes will
/// be the row-column coordinates of the open cells, and each node will have
/// weight-1 edges to its open four-directional neighbors.
pub fn from_bool_grid(grid: &Grid<bool>) -> Digraph<Point> {
	let mut digraph = Digraph::new();
	for (node, &open) in grid {
		if !open {
			continue;
		}
		for (neighbor, &neighbor_open) in grid.four_neighbors(node) {
			if neighbor_open {
				digraph.insert_edge(node, neighbor, 1);
			}
		}
	}
	digraph
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
	fn digraph_all_to_all_shortest_distances() {
		let d = new_test_digraph().all_to_all_shortest_distances();
		assert_eq!(d.weight("start", "start"), Some(0));
		assert_eq!(d.weight("start", "goal"), Some(3));
		assert_eq!(d.weight("a", "c"), Some(3));
		assert_eq!(d.weight("goal", "goal"), Some(0));
		assert!(d.weight("goal", "start").is_none());
	}

	#[test]
	fn graph_all_to_all_shortest_distances() {
		let d = new_test_graph().all_to_all_shortest_distances();
		assert_eq!(d.weight("start", "start"), Some(0));
		assert_eq!(d.weight("start", "goal"), Some(3));
		assert_eq!(d.weight("a", "c"), Some(3));
		assert_eq!(d.weight("goal", "goal"), Some(0));
		assert_eq!(d.weight("goal", "start"), Some(3));
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
