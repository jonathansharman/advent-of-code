use aoc::grid::{Grid, Point, Vector};

aoc::test::test_part!(test1, part1, 432);
aoc::test::test_part!(test2, part2, "56,27");

const INPUT: &str = include_str!("input.txt");
const GRID_SIZE: Vector = Vector::new(71, 71);
const BYTE_COUNT: usize = 1024;

pub fn part1() -> usize {
	let mut grid = Grid::new(GRID_SIZE, true);
	INPUT
		.lines()
		.map(|line| {
			let (col, row) = line.split_once(',').unwrap();
			Point::new(row.parse().unwrap(), col.parse().unwrap())
		})
		.take(BYTE_COUNT)
		.for_each(|point| grid[point] = false);
	grid.bfs_four(
		Point::zero(),
		Point::new(grid.row_count() - 1, grid.col_count() - 1),
		|coords| !grid[coords],
	)
	.unwrap()
}

pub fn part2() -> String {
	let mut grid = Grid::new(GRID_SIZE, true);
	let point = INPUT
		.lines()
		.map(|line| {
			let (col, row) = line.split_once(',').unwrap();
			Point::new(row.parse().unwrap(), col.parse().unwrap())
		})
		.find(|&point| {
			grid[point] = false;
			grid.bfs_four(
				Point::zero(),
				Point::new(grid.row_count() - 1, grid.col_count() - 1),
				|coords| !grid[coords],
			)
			.is_none()
		})
		.unwrap();
	format!("{},{}", point.col, point.row)
}
