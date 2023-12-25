use crate::{io::read_lines, neighbors};
use itertools::Itertools;
use std::collections::BinaryHeap;

crate::test::test_part!(test1, part1, 435);
crate::test::test_part!(test2, part2, 2842);

pub fn part1() -> u32 {
	dijkstra(read_maze())
}

pub fn part2() -> u32 {
	dijkstra(expand_maze(read_maze()))
}

fn read_maze() -> Vec<Vec<u32>> {
	read_lines("input/2021/15.txt")
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
		.collect_vec()
}

fn expand_maze(maze: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
	let n = maze.len();
	let mut expanded = vec![vec![0; 5 * n]; 5 * n];
	for (i, row) in maze.into_iter().enumerate() {
		for (j, cost) in row.into_iter().enumerate() {
			for k in 0..5 {
				for l in 0..5 {
					expanded[i + n * k as usize][j + n * l as usize] =
						(cost + k + l - 1) % 9 + 1;
				}
			}
		}
	}
	expanded
}

#[derive(PartialEq, Eq)]
struct Node {
	i: usize,
	j: usize,
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

fn dijkstra(maze: Vec<Vec<u32>>) -> u32 {
	let n = maze.len();

	let mut ds = vec![vec![u32::MAX; n]; n];
	ds[0][0] = 0;

	let mut queue = BinaryHeap::new();
	queue.push(Node { i: 0, j: 0, d: 0 });

	while let Some(Node { i, j, d }) = queue.pop() {
		if ds[i][j] < d {
			continue;
		}

		for (i, j) in neighbors::four(n, n, i, j) {
			let d_new = d.saturating_add(maze[i][j]);
			if d_new < ds[i][j] {
				ds[i][j] = d_new;
				queue.push(Node { i, j, d: d_new });
			}
		}
	}
	ds[n - 1][n - 1]
}
