use std::collections::HashSet;

use aoc::{
	graph::Digraph,
	grid::{Point, Vector, COMPASS, EAST},
	input::{input, ParseGrid},
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
	let grid =
		input!()
			.parse_grid(|c| if c == '#' { Tile::Wall } else { Tile::Floor });

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
		.shortest_distance(maze.start, |&state| state.coords == maze.end)
		.unwrap()
}

pub fn part2() -> usize {
	let Maze { graph, start, end } = read_maze();
	// Find the lowest-cost paths from the start state to all other states.
	let paths = graph.one_to_all_shortest_paths(start);
	// Get the lowest cost of any path to a state with the target end
	// coordinates, facing any direction.
	let lowest_cost = paths.shortest_distance(|state| state.coords == end);
	graph
		.nodes()
		.iter()
		.filter(|state| {
			// Some states with the correct coordinates may be facing the wrong
			// direction - and thus be reachable with a suboptimal number of
			// points. Only consider states that are reachable in the globally
			// lowest number of points.
			state.coords == end && paths.distance(state) == lowest_cost
		})
		.flat_map(|&state| {
			// Trace each of these states' shortest paths back to the start, and
			// disregard their directions.
			paths
				.backtrace(state)
				.into_nodes()
				.into_iter()
				.map(|state| state.coords)
		})
		.collect::<HashSet<_>>()
		.len()
}
