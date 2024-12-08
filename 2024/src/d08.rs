use std::collections::{HashMap, HashSet};

use aoc::io::read_lines;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, ?);
aoc::test::test_part!(test2, part2, ?);

type Point = (i32, i32);

struct Map {
	nodes: Vec<Vec<Option<char>>>,
	height: i32,
	width: i32,
}

fn read_map() -> Map {
	let nodes: Vec<Vec<Option<char>>> = read_lines("input/08.txt")
		.map(|line| line.chars().map(|c| (c != '.').then_some(c)).collect())
		.collect();
	let height = nodes.len() as i32;
	let width = nodes[0].len() as i32;
	Map {
		nodes,
		height,
		width,
	}
}

impl Map {
	fn node_locations(&self) -> HashMap<char, Vec<Point>> {
		let mut node_locations: HashMap<char, Vec<Point>> = HashMap::new();
		for (i, line) in self.nodes.iter().enumerate() {
			for (j, c) in line.iter().enumerate() {
				if let Some(c) = c {
					node_locations
						.entry(*c)
						.or_default()
						.push((i as i32, j as i32));
				}
			}
		}
		node_locations
	}

	fn antinode_locations(&self) -> HashSet<Point> {
		let mut antinode_locations = HashSet::new();
		for locations in self.node_locations().values() {
			locations
				.iter()
				.enumerate()
				.cartesian_product(locations.iter().enumerate())
				.filter_map(|((i, p1), (j, p2))| {
					let p3 = (2 * p2.0 - p1.0, 2 * p2.1 - p1.1);
					(i != j
						&& (0..self.height).contains(&p3.0)
						&& (0..self.width).contains(&p3.1))
					.then_some(p3)
				})
				.for_each(|p| {
					antinode_locations.insert(p);
				});
		}
		antinode_locations
	}
}

pub fn part1() -> usize {
	read_map().antinode_locations().len()
}

pub fn part2() -> usize {
	0
}
