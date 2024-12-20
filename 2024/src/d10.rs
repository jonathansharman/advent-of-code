use std::collections::HashSet;

use aoc::{
	graph::Digraph,
	grid::{Grid, Point},
};

aoc::test::test_part!(test1, part1, 688);
aoc::test::test_part!(test2, part2, 1459);

const INPUT: &str = include_str!("input/10.txt");

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
	let grid: Grid<_> = INPUT
		.lines()
		.map(|line| line.bytes().map(|b| b - b'0').collect())
		.collect();
	let mut trails = Digraph::new();
	let mut trailheads = HashSet::new();
	let mut peaks = HashSet::new();
	for (node, height) in &grid {
		match height {
			0 => {
				trailheads.insert(node);
			}
			9 => {
				peaks.insert(node);
			}
			_ => {}
		}
		for (neighbor, &neighbor_height) in grid.four_neighbors(node) {
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
