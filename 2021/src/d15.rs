use aoc::{
	grid::{Grid, Point},
	io::read_lines,
};
use std::collections::BinaryHeap;

aoc::test::test_part!(test1, part1, 435);
aoc::test::test_part!(test2, part2, 2842);

pub fn part1() -> u32 {
	dijkstra(read_maze())
}

pub fn part2() -> u32 {
	dijkstra(expand_maze(read_maze()))
}

fn read_maze() -> Grid<u32> {
	read_lines("input/15.txt")
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
		.collect()
}

fn expand_maze(maze: Grid<u32>) -> Grid<u32> {
	let n = maze.row_count() as usize;
	let mut expanded = Grid::new(5 * maze.size(), 0);
	for (i, row) in maze.rows().enumerate() {
		for (j, cost) in row.enumerate() {
			for k in 0..5 {
				for l in 0..5 {
					expanded[(i + n * k as usize, j + n * l as usize).into()] =
						(cost + k + l - 1) % 9 + 1;
				}
			}
		}
	}
	expanded
}

#[derive(PartialEq, Eq)]
struct Node {
	coords: Point,
	d: u32,
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.d.cmp(&other.d).reverse()
	}
}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

fn dijkstra(maze: Grid<u32>) -> u32 {
	let n = maze.row_count() as usize;

	let mut ds = Grid::new(maze.size(), u32::MAX);
	ds[Point::zero()] = 0;

	let mut queue = BinaryHeap::new();
	queue.push(Node {
		coords: Point::zero(),
		d: 0,
	});

	while let Some(Node { coords, d }) = queue.pop() {
		if ds[coords] < d {
			continue;
		}

		for (neighbor_coords, &d_neighbor) in maze.four_neighbors(coords) {
			let d_new = d.saturating_add(d_neighbor);
			if d_new < ds[neighbor_coords] {
				ds[neighbor_coords] = d_new;
				queue.push(Node {
					coords: neighbor_coords,
					d: d_new,
				});
			}
		}
	}
	ds[(n - 1, n - 1).into()]
}
