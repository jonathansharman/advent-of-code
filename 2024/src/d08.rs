use std::{
	collections::{HashMap, HashSet},
	ops::RangeBounds,
};

use aoc::io::read_lines;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 423);
aoc::test::test_part!(test2, part2, 1287);

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
		self.nodes
			.iter()
			.enumerate()
			.flat_map(|(i, line)| {
				line.iter().enumerate().filter_map(move |(j, c)| {
					c.map(|c| (c, (i as i32, j as i32)))
				})
			})
			.fold(
				HashMap::new(),
				|mut acc: HashMap<char, Vec<Point>>, (node, location)| {
					acc.entry(node).or_default().push(location);
					acc
				},
			)
	}

	fn antinode_locations(
		&self,
		range: impl RangeBounds<i32> + IntoIterator<Item = i32> + Clone,
	) -> HashSet<Point> {
		self.node_locations()
			.values()
			.flat_map(|locations| {
				locations
					.iter()
					.enumerate()
					.cartesian_product(locations.iter().enumerate())
					.filter(|((i, _), (j, _))| i != j)
					.flat_map(|((_, p1), (_, p2))| {
						range.clone().into_iter().map_while(|offset| {
							let p3 = (
								p2.0 + offset * (p2.0 - p1.0),
								p2.1 + offset * (p2.1 - p1.1),
							);
							((0..self.height).contains(&p3.0)
								&& (0..self.width).contains(&p3.1))
							.then_some(p3)
						})
					})
			})
			.collect()
	}
}

pub fn part1() -> usize {
	read_map().antinode_locations(1..=1).len()
}

pub fn part2() -> usize {
	read_map().antinode_locations(0..).len()
}
