use aoc::{
	graph::{self, DijkstraResults},
	grid::{Grid, NORTH, Point, Vector, WEST},
	input,
};

aoc::test::test_part!(test1, part1, 1422);
aoc::test::test_part!(test2, part2, ?);

const MIN_SAVINGS: usize = 100;
const MAX_CHEAT_TIME: i64 = 20;

struct Maze {
	grid: Grid<bool>,
	to_end: DijkstraResults<Point>,
}

fn read_maze() -> Maze {
	let mut end = Point::zero();
	let grid: Grid<bool> = input!()
		.lines()
		.enumerate()
		.map(|(i, line)| {
			line.chars()
				.enumerate()
				.map(|(j, c)| match c {
					'.' | 'S' => true,
					'E' => {
						end = (i, j).into();
						true
					}
					_ => false,
				})
				.collect::<Vec<_>>()
		})
		.collect();
	let graph = graph::from_bool_grid(&grid);
	let to_end = graph.one_to_all_shortest_paths(end);
	Maze { grid, to_end }
}

pub fn part1() -> usize {
	let maze = read_maze();
	let mut cheats = 0;
	for (coords, &open) in &maze.grid {
		// Only consider walls.
		if open {
			continue;
		}
		for offset in [NORTH, WEST] {
			// Get opposing neighbors.
			let nc1 = coords + offset;
			let nc2 = coords - offset;
			let (Some(&n1), Some(&n2)) =
				(maze.grid.get(nc1), maze.grid.get(nc2))
			else {
				continue;
			};
			// Both neighbors must be open.
			if !n1 || !n2 {
				continue;
			}
			// Both neighbors must be able to reach the end.
			let (Some(d1), Some(d2)) =
				(maze.to_end.distance(&nc1), maze.to_end.distance(&nc2))
			else {
				continue;
			};
			// The difference must be two greater than the minimum required
			// savings, to account for moving onto and out of the wall.
			if d1.abs_diff(d2) >= MIN_SAVINGS + 2 {
				cheats += 1;
			}
		}
	}
	cheats
}

pub fn part2() -> usize {
	let maze = read_maze();
	let mut cheats = 0;
	for (start, &start_open) in &maze.grid {
		// Only consider floors.
		if !start_open {
			continue;
		}
		// This tile must be able to reach the end.
		let Some(d1) = maze.to_end.distance(&start) else {
			continue;
		};
		// Consider all tiles reachable in up to the max amount of cheat time.
		for row_offset in -MAX_CHEAT_TIME..=MAX_CHEAT_TIME {
			let max_col_offset = MAX_CHEAT_TIME - row_offset.abs();
			for col_offset in -max_col_offset..=max_col_offset {
				let end = start + Vector::new(row_offset, col_offset);
				let Some(end_open) = maze.grid.get(end) else {
					continue;
				};
				// Must land on a floor tile.
				if !end_open {
					continue;
				}
				// The other tile must be able to reach the end.
				let Some(d2) = maze.to_end.distance(&end) else {
					continue;
				};
				// The time savings must meet the minimum required.
				let d = (row_offset.abs() + col_offset.abs()) as usize;
				if d1.saturating_sub(d2).saturating_sub(d) >= MIN_SAVINGS {
					cheats += 1;
				}
			}
		}
	}
	cheats
}
