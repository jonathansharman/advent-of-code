use std::collections::{hash_map::Entry, BinaryHeap, HashMap};

pub trait Node: Copy + Eq + std::hash::Hash {}

impl<T> Node for T where T: Copy + Eq + std::hash::Hash {}

/// An outgoing graph edge.
pub struct Edge<T: Node> {
	neighbor: T,
	cost: usize,
}

impl<T: Node> Edge<T> {
	/// Creates an edge to the given `neighbor` with the given `cost`.
	pub fn new(neighbor: T, cost: usize) -> Self {
		Self { neighbor, cost }
	}
}

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

#[derive(Default)]
pub struct Graph<T: Node> {
	edges: HashMap<T, Vec<Edge<T>>>,
}

impl<T: Node> FromIterator<(T, Vec<Edge<T>>)> for Graph<T> {
	/// Creates a `Graph` from an iterator of (node, edge) pairs.
	fn from_iter<I: IntoIterator<Item = (T, Vec<Edge<T>>)>>(iter: I) -> Self {
		Self {
			edges: HashMap::from_iter(iter),
		}
	}
}

impl<T: Node> Graph<T> {
	/// Creates a new empty `Graph`.
	pub fn new() -> Self {
		Self {
			edges: HashMap::new(),
		}
	}

	/// The entry for the edges from the given `node`.
	pub fn edges(&mut self, node: T) -> Entry<'_, T, Vec<Edge<T>>> {
		self.edges.entry(node)
	}

	/// The shortest distance from `start` to a node that satisfies `pred` or
	/// `None` if no such node is reachable.
	pub fn shortest_distance<F>(&self, start: T, pred: F) -> Option<usize>
	where
		F: Fn(T) -> bool,
	{
		let mut distance_map: HashMap<T, usize> = HashMap::new();
		let mut queue: BinaryHeap<State<T>> = BinaryHeap::new();
		distance_map.insert(start, 0);
		queue.push(State {
			distance: 0,
			node: start,
		});
		while let Some(State { distance, node }) = queue.pop() {
			if pred(node) {
				return Some(distance);
			}
			if distance > distance_map[&node] {
				// We've already reached this node by a shorter path.
				continue;
			}
			for Edge { neighbor, cost } in self.edges[&node].iter() {
				let candidate = distance + cost;
				let d = distance_map.get(neighbor);
				if d.map_or(true, |d| candidate < *d) {
					queue.push(State {
						distance: candidate,
						node: *neighbor,
					});
					distance_map.insert(*neighbor, candidate);
				}
			}
		}
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn shortest_distance() {
		let graph = Graph::from_iter([
			("start", vec![Edge::new("a", 1), Edge::new("shortcut", 2)]),
			("a", vec![Edge::new("b", 1)]),
			("b", vec![Edge::new("c", 1)]),
			("c", vec![Edge::new("goal", 1)]),
			("shortcut", vec![Edge::new("goal", 1)]),
		]);
		let d = graph.shortest_distance("start", |n| n == "goal");
		assert_eq!(d, Some(3));
	}
}
