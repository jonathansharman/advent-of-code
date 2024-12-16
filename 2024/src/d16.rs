use std::collections::HashSet;

use aoc::{
	graph::Digraph,
	grid::{Point, Vector, COMPASS, EAST},
	io::read_grid,
};

aoc::test::test_part!(test1, part1, 78428);
aoc::test::test_part!(test2, part2, 463);

#[derive(Clone, Copy)]
enum Tile {
	Wall,
	Floor,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

struct Maze {
	graph: Digraph<State>,
	start: State,
	end: Point,
}

fn read_maze() -> Maze {
	let grid = read_grid("input/16.txt", |c| {
		if c == '#' {
			Tile::Wall
		} else {
			Tile::Floor
		}
	});

	let start = State {
		coords: Point {
			row: grid.row_count() - 2,
			col: 1,
		},
		direction: EAST,
	};
	let end = Point {
		row: 1,
		col: grid.col_count() - 2,
	};

	let mut graph = Digraph::new();
	for (coords, tile) in &grid {
		if let Tile::Floor = tile {
			for forward in COMPASS {
				const MOVE_COST: usize = 1;
				const TURN_COST: usize = 1000;

				if let Tile::Floor = grid[coords + forward] {
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
				if let Tile::Floor = grid[coords + left] {
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
				if let Tile::Floor = grid[coords + right] {
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

	Maze { graph, start, end }
}

pub fn part1() -> usize {
	let maze = read_maze();
	maze.graph
		.shortest_distance(maze.start, |&node| node.coords == maze.end)
		.unwrap()
}

pub fn part2() -> usize {
	let Maze { graph, start, end } = read_maze();
	let d_start_end = graph
		.shortest_distance(start, |&node| node.coords == end)
		.unwrap();
	graph
		.get_nodes()
		.into_iter()
		.filter_map(|waypoint| {
			let d_start_waypoint =
				graph.shortest_distance(start, |&node| node == waypoint)?;
			// Short-circuit if we're already overbudget.
			if d_start_waypoint > d_start_end {
				return None;
			}
			let d_waypoint_end = graph
				.shortest_distance(waypoint, |&node| node.coords == end)?;
			let d_start_waypoint_end = d_start_waypoint + d_waypoint_end;
			(d_start_waypoint_end == d_start_end).then_some(waypoint.coords)
		})
		.collect::<HashSet<_>>()
		.len()
}
