use std::{
	collections::HashSet,
	fmt::{Debug, Write},
};

use aoc::{
	grid::{Grid, Point, Vector},
	io::read_lines,
};

aoc::test::test_part!(test1, part1, 1475249);
aoc::test::test_part!(test2, part2, 1509724);
// 756814 too low
// 1196504 too low

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
	Robot,
	Floor,
	Box,
	LeftBox,
	RightBox,
	Wall,
}

impl Debug for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let c = match self {
			Tile::Robot => '@',
			Tile::Floor => '.',
			Tile::Box => 'O',
			Tile::LeftBox => '[',
			Tile::RightBox => ']',
			Tile::Wall => '#',
		};
		f.write_char(c)
	}
}

struct Warehouse {
	robot: Point,
	tiles: Grid<Tile>,
}

impl Warehouse {
	fn expand(self) -> Warehouse {
		let robot = Point {
			row: self.robot.row,
			col: 2 * self.robot.col,
		};
		let tiles = self
			.tiles
			.rows()
			.map(|row| {
				row.flat_map(|tile| match tile {
					Tile::Robot => [Tile::Robot, Tile::Floor],
					Tile::Floor => [Tile::Floor; 2],
					Tile::Box => [Tile::LeftBox, Tile::RightBox],
					Tile::Wall => [Tile::Wall; 2],
					_ => panic!("inexpansible tile"),
				})
				.collect()
			})
			.collect();
		Warehouse { robot, tiles }
	}

	/// Attempts to move the robot towards `offset`, which must be a unit
	/// vector.
	fn move_robot(&mut self, offset: Vector) {
		if let Some(to_move) =
			self.push(&mut HashSet::new(), self.robot, offset)
		{
			for box_coords in to_move {
				self.tiles.swap(box_coords, box_coords + offset);
			}
			self.robot += offset;
		}
	}

	/// Returns the series of coordinates, in order, whose occupants would also
	/// need to move in order to clear the space at `coords` towards `offset`
	/// (`offset` must be a unit vector). If there's no way to clear the space,
	/// this returns `None`. If `coords` is in `visited`, this returns an empty
	/// list.
	fn push(
		&mut self,
		visited: &mut HashSet<Point>,
		coords: Point,
		offset: Vector,
	) -> Option<Vec<Point>> {
		if !visited.insert(coords) {
			return Some(Vec::new());
		}
		self.tiles.get(coords).copied().and_then(|tile| match tile {
			Tile::Floor => Some(Vec::new()),
			Tile::Robot | Tile::Box => self
				.push(visited, coords + offset, offset)
				.map(|mut to_move| {
					to_move.push(coords);
					to_move
				}),
			Tile::LeftBox => {
				if offset.col != 0 {
					// Treat left-right pushes just like a normal box.
					return self.push(visited, coords + offset, offset).map(
						|mut to_move| {
							to_move.push(coords);
							to_move
						},
					);
				}
				// Vertical pushes can cause a tree of additional pushes.
				let right_coords = coords + Vector::new(0, 1);
				self.push(visited, right_coords + offset, offset).and_then(
					|mut to_move| {
						if !visited.contains(&right_coords) {
							to_move.push(right_coords);
						}
						self.push(visited, coords + offset, offset).map(
							|left_to_move| {
								to_move.extend(left_to_move);
								if !visited.contains(&right_coords) {
									to_move.push(coords);
								}
								to_move
							},
						)
					},
				)
			}
			Tile::RightBox => {
				if offset.col != 0 {
					// Treat left-right pushes just like a normal box.
					return self.push(visited, coords + offset, offset).map(
						|mut to_move| {
							to_move.push(coords);
							to_move
						},
					);
				}
				// Vertical pushes can cause a tree of additional pushes.
				let left_coords = coords - Vector::new(0, 1);
				self.push(visited, left_coords + offset, offset).and_then(
					|mut to_move| {
						if !visited.contains(&left_coords) {
							to_move.push(left_coords);
						}
						self.push(visited, coords + offset, offset).map(
							|right_to_move| {
								to_move.extend(right_to_move);
								if !visited.contains(&left_coords) {
									to_move.push(coords);
								}
								to_move
							},
						)
					},
				)
			}
			Tile::Wall => None,
		})
	}

	fn total_gps(self) -> i64 {
		self.tiles
			.iter()
			.filter_map(|(coords, tile)| {
				(matches!(tile, Tile::Box | Tile::LeftBox))
					.then_some(100 * coords.row + coords.col)
			})
			.sum()
	}
}

fn read() -> (Warehouse, Vec<Vector>) {
	let mut lines = read_lines("input/15.txt");
	let mut robot = Point::zero();
	let tiles = lines
		.by_ref()
		.enumerate()
		.map_while(|(i, line)| {
			(!line.is_empty()).then(|| {
				line.chars()
					.enumerate()
					.map(|(j, c)| match c {
						'@' => {
							robot = (i, j).into();
							Tile::Robot
						}
						'.' => Tile::Floor,
						'O' => Tile::Box,
						'#' => Tile::Wall,
						_ => panic!("invalid tile"),
					})
					.collect()
			})
		})
		.collect();
	let movements = lines
		.flat_map(|line| {
			line.chars()
				.map(|b| match b {
					'^' => Vector::new(-1, 0),
					'v' => Vector::new(1, 0),
					'<' => Vector::new(0, -1),
					'>' => Vector::new(0, 1),
					_ => panic!("invalid movement"),
				})
				.collect::<Vec<_>>()
		})
		.collect();
	(Warehouse { robot, tiles }, movements)
}

pub fn part1() -> i64 {
	let (mut warehouse, movements) = read();
	for movement in movements {
		warehouse.move_robot(movement);
	}
	warehouse.total_gps()
}

pub fn part2() -> i64 {
	let (warehouse, movements) = read();
	let mut warehouse = warehouse.expand();
	for movement in movements {
		warehouse.move_robot(movement);
	}
	warehouse.total_gps()
}
