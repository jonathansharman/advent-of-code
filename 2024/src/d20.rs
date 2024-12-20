use aoc::{
	graph,
	grid::{Grid, Point, NORTH, WEST},
	io::read_lines,
};

aoc::test::test_part!(test1, part1, 1422);
aoc::test::test_part!(test2, part2, ?);

const MIN_SAVINGS: usize = 100;

pub fn part1() -> usize {
	let mut start = Point::zero();
	let mut end = Point::zero();
	let grid: Grid<bool> = read_lines("input/20.txt")
		.enumerate()
		.map(|(i, line)| {
			line.chars()
				.enumerate()
				.map(|(j, c)| match c {
					'S' => {
						start = (i, j).into();
						true
					}
					'E' => {
						end = (i, j).into();
						true
					}
					'.' => true,
					_ => false,
				})
				.collect()
		})
		.collect();
	let graph = graph::from_bool_grid(&grid);
	let paths = graph.one_to_all_shortest_paths(end);
	let mut cheats = 0;
	for (coords, &tile) in &grid {
		// Only consider walls.
		if tile {
			continue;
		}
		for offset in [NORTH, WEST] {
			// Get opposing neighbors.
			let nc1 = coords + offset;
			let nc2 = coords - offset;
			let (Some(&n1), Some(&n2)) = (grid.get(nc1), grid.get(nc2)) else {
				continue;
			};
			// Both neighbors must be open.
			if !n1 || !n2 {
				continue;
			}
			// Both neighbors must be able to reach the end.
			let (Some(d1), Some(d2)) =
				(paths.distance(&nc1), paths.distance(&nc2))
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
