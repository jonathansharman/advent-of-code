use std::collections::HashSet;

use aoc::{
	graph::Digraph,
	grid::{Grid, Point},
	io::read_lines,
};

aoc::test::test_part!(test1, part1, 688);
aoc::test::test_part!(test2, part2, 1459);

struct Map {
	trails: Digraph<Point>,
	trailheads: HashSet<Point>,
	peaks: HashSet<Point>,
}

impl Map {
	fn rating(&self, start: Point) -> usize {
		if self.peaks.contains(&start) {
			return 1;
		}
		match self.trails.edges_from(&start) {
			Some(edges) => {
				edges.keys().map(|&neighbor| self.rating(neighbor)).sum()
			}
			None => 0,
		}
	}
}

fn read_map() -> Map {
	let map = Grid::from_iter(
		read_lines("input/10.txt")
			.map(|line| line.bytes().map(|b| b - b'0').collect()),
	);
	let mut trails = Digraph::new();
	let mut trailheads = HashSet::new();
	let mut peaks = HashSet::new();
	for (node, height) in map.tiles() {
		match height {
			0 => {
				trailheads.insert(node);
			}
			9 => {
				peaks.insert(node);
			}
			_ => {}
		}
		for (neighbor, &neighbor_height) in map.four_neighbors(node) {
			if neighbor_height == height + 1 {
				trails.insert_edge(node, neighbor, 1);
			}
		}
	}
	Map {
		trails,
		trailheads,
		peaks,
	}
}

pub fn part1() -> usize {
	let map = read_map();
	map.trailheads
		.into_iter()
		.map(|trailhead| {
			map.peaks
				.iter()
				.filter(|&peak| {
					map.trails
						.shortest_distance(trailhead, |node| node == peak)
						.is_some()
				})
				.count()
		})
		.sum()
}

pub fn part2() -> usize {
	let map = read_map();
	map.trailheads
		.iter()
		.map(|&trailhead| map.rating(trailhead))
		.sum()
}
