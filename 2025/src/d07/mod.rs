use std::fmt::Debug;

use aoc::{
	grid::{Point, Vector},
	input,
	input::ParseGrid,
};

aoc::test::test_part!(test1, part1, 1717);
aoc::test::test_part!(test2, part2, ?);

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

pub fn part1() -> usize {
	let mut diagram = input!().parse_grid(|c| match c {
		'S' => Tile::Beam,
		'^' => Tile::Splitter,
		_ => Tile::Space,
	});
	let mut splits = 0;
	for i in 1..diagram.row_count() {
		for j in 0..diagram.col_count() {
			if diagram[(i - 1, j).into()] != Tile::Beam {
				continue;
			}
			let coords = Point::new(i, j);
			if let Tile::Splitter = diagram[coords] {
				splits += 1;
				diagram[coords - Vector::new(0, 1)] = Tile::Beam;
				diagram[coords + Vector::new(0, 1)] = Tile::Beam;
			} else {
				diagram[coords] = Tile::Beam;
			}
		}
	}
	splits
}

pub fn part2() -> usize {
	0
}
