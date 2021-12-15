use crate::io::read_lines;
use itertools::Itertools;
use std::collections::HashSet;

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
		for (j, space) in row.into_iter().enumerate() {
			for k in 0..5 {
				for l in 0..5 {
					expanded[i + n * k as usize][j + n * l as usize] = (space + k + l - 1) % 9 + 1;
				}
			}
		}
	}
	expanded
}

fn dijkstra(maze: Vec<Vec<u32>>) -> u32 {
	let n = maze.len();
	let mut distances = vec![vec![u32::MAX; n]; n];
	distances[0][0] = 0;
	let mut unvisited: HashSet<(usize, usize)> = HashSet::new();
	for i in 0..n {
		for j in 0..n {
			unvisited.insert((i, j));
		}
	}
	while !unvisited.is_empty() {
		let (i, j) = *unvisited
			.iter()
			.min_by(|(i1, j1), (i2, j2)| distances[*i1][*j1].cmp(&distances[*i2][*j2]))
			.unwrap();
		unvisited.remove(&(i, j));

		if i > 0 && unvisited.contains(&(i - 1, j)) {
			distances[i - 1][j] =
				distances[i - 1][j].min(distances[i][j].saturating_add(maze[i - 1][j]));
		}
		if i < n - 1 && unvisited.contains(&(i + 1, j)) {
			distances[i + 1][j] =
				distances[i + 1][j].min(distances[i][j].saturating_add(maze[i + 1][j]));
		}
		if j > 0 && unvisited.contains(&(i, j - 1)) {
			distances[i][j - 1] =
				distances[i][j - 1].min(distances[i][j].saturating_add(maze[i][j - 1]));
		}
		if j < n - 1 && unvisited.contains(&(i, j + 1)) {
			distances[i][j + 1] =
				distances[i][j + 1].min(distances[i][j].saturating_add(maze[i][j + 1]));
		}
	}
	distances[n - 1][n - 1]
}
