use aoc::{
	graph::Digraph,
	grid::{Point, Vector, COMPASS, EAST},
	io::read_grid,
};

aoc::test::test_part!(test1, part1, 78428);
aoc::test::test_part!(test2, part2, ?);

#[derive(Clone, Copy)]
enum Tile {
	Wall,
	Floor,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
	coords: Point,
	direction: Vector,
}

fn rotate_cw(v: Vector) -> Vector {
	Vector {
		row: v.col,
		col: -v.row,
	}
}

fn rotate_ccw(v: Vector) -> Vector {
	Vector {
		row: -v.col,
		col: v.row,
	}
}

pub fn part1() -> usize {
	let maze = read_grid("input/16.txt", |c| {
		if c == '#' {
			Tile::Wall
		} else {
			Tile::Floor
		}
	});

	let start = State {
		coords: Point {
			row: maze.row_count() - 2,
			col: 1,
		},
		direction: EAST,
	};
	let end = Point {
		row: 1,
		col: maze.col_count() - 2,
	};

	let mut graph = Digraph::new();
	for (coords, tile) in &maze {
		if let Tile::Floor = tile {
			for forward in COMPASS {
				const MOVE_COST: usize = 1;
				const TURN_COST: usize = 1000;

				if let Tile::Floor = maze[coords + forward] {
					graph.insert_edge(
						State {
							coords,
							direction: forward,
						},
						State {
							coords: coords + forward,
							direction: forward,
						},
						MOVE_COST,
					);
				}

				let left = rotate_ccw(forward);
				if let Tile::Floor = maze[coords + left] {
					graph.insert_edge(
						State {
							coords,
							direction: forward,
						},
						State {
							coords,
							direction: left,
						},
						TURN_COST,
					);
				}

				let right = rotate_cw(forward);
				if let Tile::Floor = maze[coords + right] {
					graph.insert_edge(
						State {
							coords,
							direction: forward,
						},
						State {
							coords,
							direction: right,
						},
						TURN_COST,
					);
				}
			}
		}
	}
	graph
		.shortest_distance(start, |&node| node.coords == end)
		.unwrap()
}

pub fn part2() -> usize {
	0
}
