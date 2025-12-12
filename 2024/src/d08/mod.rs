use std::{
	collections::{HashMap, HashSet},
	ops::RangeBounds,
};

use aoc::{
	grid::{Grid, Point},
	input,
};
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 423);
aoc::test::test_part!(test2, part2, 1287);

type Map = Grid<Option<char>>;

fn read_map() -> Map {
	input!()
		.lines()
		.map(|line| line.chars().map(|c| (c != '.').then_some(c)))
		.collect()
}

fn node_locations(map: &Map) -> HashMap<char, Vec<Point>> {
	map.rows()
		.enumerate()
		.flat_map(|(i, row)| {
			row.enumerate()
				.filter_map(move |(j, c)| c.map(|c| (c, Point::from((i, j)))))
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
	map: &Map,
	range: impl RangeBounds<i64> + IntoIterator<Item = i64> + Clone,
) -> HashSet<Point> {
	node_locations(map)
		.values()
		.flat_map(|locations| {
			locations
				.iter()
				.enumerate()
				.cartesian_product(locations.iter().enumerate())
				.filter(|((i, _), (j, _))| i != j)
				.flat_map(|((_, &p1), (_, &p2))| {
					range.clone().into_iter().map_while(move |offset| {
						let p3 = p2 + offset * (p2 - p1);
						map.contains_coords(p3).then_some(p3)
					})
				})
		})
		.collect()
}

pub fn part1() -> usize {
	antinode_locations(&read_map(), 1..=1).len()
}

pub fn part2() -> usize {
	antinode_locations(&read_map(), 0..).len()
}
