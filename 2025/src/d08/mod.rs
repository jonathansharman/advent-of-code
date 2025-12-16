use std::collections::BinaryHeap;

use aoc::{graph::Graph, input};
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 68112);
aoc::test::test_part!(test2, part2, ?);

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

pub fn part1() -> usize {
	let boxes: Vec<Point> = input!()
		.lines()
		.map(|line| {
			line.split(',')
				.map(|n| n.parse().unwrap())
				.collect_array()
				.unwrap()
		})
		.collect();

	let mut closest_edges = BinaryHeap::new();
	for (i, &b1) in boxes.iter().enumerate() {
		for &b2 in boxes.iter().skip(i + 1) {
			closest_edges.push(Edge(b1, b2));
		}
	}

	let mut circuits = Graph::new();
	for _ in 0..1000 {
		let Some(edge) = closest_edges.pop() else {
			break;
		};
		println!("connecting {:?} and {:?}", edge.0, edge.1);
		circuits.insert_edge(edge.0, edge.1, 1);
	}

	let mut largest_circuits = circuits
		.into_connected_components()
		.into_iter()
		.map(|circuit| {
			println!("circuit of size {}", circuit.nodes().len());
			circuit.nodes().len()
		})
		.collect::<BinaryHeap<_>>();

	(0..3)
		.map(|_| {
			let size = largest_circuits.pop().unwrap();
			println!("top-3 circuit of size {size}");
			size
		})
		.product()
}

pub fn part2() -> usize {
	0
}
