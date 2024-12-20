use aoc::{
	graph::{self, DijkstraResults},
	grid::{Grid, Point, Vector, NORTH, WEST},
};

aoc::test::test_part!(test1, part1, 1422);
aoc::test::test_part!(test2, part2, ?);

const INPUT: &str = include_str!("input/20.txt");
const MIN_SAVINGS: usize = 100;

struct Maze {
	grid: Grid<bool>,
	to_end: DijkstraResults<Point>,
}

fn read_maze() -> Maze {
	let mut end = Point::zero();
	let grid: Grid<bool> = INPUT
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
				.collect()
		})
		.collect();
	let graph = graph::from_bool_grid(&grid);
	let to_end = graph.one_to_all_shortest_paths(end);
	Maze { grid, to_end }
}

pub fn part1() -> usize {
	let maze = read_maze();
	let mut cheats = 0;
	for (coords, &tile) in &maze.grid {
		// Only consider walls.
		if tile {
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
	0
}
