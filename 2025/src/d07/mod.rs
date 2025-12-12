use std::{collections::HashMap, fmt::Debug};

use aoc::{
	grid::{EAST, Grid, Point, SOUTH, SOUTHEAST, SOUTHWEST, WEST},
	input,
	input::ParseGrid,
};
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 1717);
aoc::test::test_part!(test2, part2, 231507396180012);

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
	Space,
	Splitter,
	Beam,
}

impl Debug for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Space => write!(f, "."),
			Self::Splitter => write!(f, "^"),
			Self::Beam => write!(f, "|"),
		}
	}
}

fn split_beams() -> Grid<Tile> {
	let mut diagram = input!().parse_grid(|c| match c {
		'S' => Tile::Beam,
		'^' => Tile::Splitter,
		_ => Tile::Space,
	});
	for i in 1..diagram.row_count() {
		for j in 0..diagram.col_count() {
			if diagram[(i - 1, j).into()] != Tile::Beam {
				continue;
			}
			let coords = Point::new(i, j);
			if let Tile::Splitter = diagram[coords] {
				diagram[coords + WEST] = Tile::Beam;
				diagram[coords + EAST] = Tile::Beam;
			} else {
				diagram[coords] = Tile::Beam;
			}
		}
	}
	diagram
}

pub fn part1() -> usize {
	let diagram = split_beams();
	diagram
		.cols()
		.map(|col| {
			col.tuple_windows()
				.filter(|&(&top, &bottom)| {
					top == Tile::Beam && bottom == Tile::Splitter
				})
				.count()
		})
		.sum()
}

fn paths(
	cache: &mut HashMap<Point, usize>,
	diagram: &Grid<Tile>,
	start: Point,
) -> usize {
	if let Some(&paths) = cache.get(&start) {
		return paths;
	}

	let paths = match diagram.get(start) {
		Some(Tile::Space) => panic!("where did the beam go?"),
		Some(Tile::Splitter) => {
			paths(cache, diagram, start + SOUTHWEST)
				+ paths(cache, diagram, start + SOUTHEAST)
		}
		Some(Tile::Beam) => paths(cache, diagram, start + SOUTH),
		None => 1,
	};
	cache.insert(start, paths);
	paths
}

pub fn part2() -> usize {
	let diagram = split_beams();
	let start = diagram
		.get_row(0)
		.unwrap()
		.position(|&tile| tile == Tile::Beam)
		.unwrap() as i64;
	paths(&mut HashMap::new(), &diagram, Point::new(0, start))
}
