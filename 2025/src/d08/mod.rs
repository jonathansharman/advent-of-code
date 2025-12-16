use std::collections::{BinaryHeap, HashSet};

use aoc::{graph::Graph, input};
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 68112);
aoc::test::test_part!(test2, part2, 44543856);

type Point = [u64; 3];

#[derive(PartialEq, Eq)]
struct Edge(Point, Point);

impl Edge {
	fn length_squared(&self) -> u64 {
		(self.1[0] - self.0[0]).pow(2)
			+ (self.1[1] - self.0[1]).pow(2)
			+ (self.1[2] - self.0[2]).pow(2)
	}
}

impl PartialOrd for Edge {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Edge {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		// Consider an edge to be less than another if its length is greater so
		// that our priority queue (using BinaryHeap, which is a max-heap) will
		// produce shorter distances first.
		other.length_squared().cmp(&self.length_squared())
	}
}

fn parse_boxes<T: FromIterator<Point>>() -> T {
	input!()
		.lines()
		.map(|line| {
			line.split(',')
				.map(|n| n.parse().unwrap())
				.collect_array()
				.unwrap()
		})
		.collect()
}

fn get_closest_edges<'a>(
	boxes: impl IntoIterator<Item = &'a Point> + Copy,
) -> BinaryHeap<Edge> {
	let mut closest_edges = BinaryHeap::new();
	for (i, &b1) in boxes.into_iter().enumerate() {
		for &b2 in boxes.into_iter().skip(i + 1) {
			closest_edges.push(Edge(b1, b2));
		}
	}
	closest_edges
}

pub fn part1() -> usize {
	let boxes: Vec<Point> = parse_boxes();
	let mut closest_edges = get_closest_edges(&boxes);

	let mut circuits = Graph::new();
	for _ in 0..1000 {
		let Some(Edge(p1, p2)) = closest_edges.pop() else {
			break;
		};
		circuits.insert_edge(p1, p2, 1);
	}

	let mut largest_circuits = circuits
		.into_connected_components()
		.into_iter()
		.map(|circuit| circuit.nodes().len())
		.collect::<BinaryHeap<_>>();
	(0..3).map(|_| largest_circuits.pop().unwrap()).product()
}

pub fn part2() -> u64 {
	let mut boxes: HashSet<Point> = parse_boxes();
	let mut closest_edges = get_closest_edges(&boxes);

	while let Some(Edge(p1, p2)) = closest_edges.pop() {
		boxes.remove(&p1);
		boxes.remove(&p2);
		if boxes.is_empty() {
			return p1[0] * p2[0];
		}
	}

	panic!("unable to connect all boxes");
}
