use aoc::{
	graph::{self},
	grid::{Grid, Point, Vector},
	io::read_lines,
};

aoc::test::test_part!(test1, part1, 432);
aoc::test::test_part!(test2, part2, "56,27");

const GRID_SIZE: Vector = Vector::new(71, 71);
const BYTE_COUNT: usize = 1024;

pub fn part1() -> usize {
	let mut grid = Grid::new(GRID_SIZE, true);
	read_lines("input/18.txt")
		.map(|line| {
			let (col, row) = line.split_once(',').unwrap();
			Point::new(row.parse().unwrap(), col.parse().unwrap())
		})
		.take(BYTE_COUNT)
		.for_each(|point| grid[point] = false);
	let graph = graph::from_bool_grid(&grid);
	graph
		.shortest_distance(Point::zero(), |&p| {
			p == Point::new(grid.row_count() - 1, grid.col_count() - 1)
		})
		.unwrap()
}

pub fn part2() -> String {
	let grid = Grid::new(GRID_SIZE, true);
	let mut graph = graph::from_bool_grid(&grid);
	let point = read_lines("input/18.txt")
		.map(|line| {
			let (col, row) = line.split_once(',').unwrap();
			Point::new(row.parse().unwrap(), col.parse().unwrap())
		})
		.find(|&point| {
			for (neighbor, _) in grid.four_neighbors(point) {
				graph.remove_edge(&point, &neighbor);
			}
			graph
				.shortest_distance(Point::zero(), |&p| {
					p == Point::new(grid.row_count() - 1, grid.col_count() - 1)
				})
				.is_none()
		})
		.unwrap();
	format!("{},{}", point.col, point.row)
}
