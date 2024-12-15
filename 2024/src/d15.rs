use aoc::{
	grid::{Grid, Point, Vector},
	io::read_lines,
};

aoc::test::test_part!(test1, part1, ?);
aoc::test::test_part!(test2, part2, ?);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
	Floor,
	Box,
	Wall,
}

struct Warehouse {
	robot: Point,
	tiles: Grid<Tile>,
}

impl Warehouse {
	/// Attempts to move the robot towards `offset`, which must be a unit
	/// vector.
	fn move_robot(&mut self, offset: Vector) {
		let target = self.robot + offset;
		if self.push(target, offset) {
			self.robot = target;
		}
	}

	/// Attempts to push any box at `coords` towards `offset`, which must be a
	/// unit vector. Returns whether `coords` is now empty (regardless of
	/// whether anything moved).
	fn push(&mut self, coords: Point, offset: Vector) -> bool {
		match self.tiles.get(coords) {
			Some(&tile) => match tile {
				Tile::Floor => true,
				Tile::Box => {
					let target = coords + offset;
					if self.push(target, offset) {
						self.tiles[target] = tile;
						self.tiles[coords] = Tile::Floor;
						true
					} else {
						false
					}
				}
				Tile::Wall => false,
			},
			None => false,
		}
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
							Tile::Floor
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
	warehouse
		.tiles
		.into_tiles()
		.filter_map(|(coords, tile)| {
			(tile == Tile::Box).then_some(100 * coords.row + coords.col)
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
