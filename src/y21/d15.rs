use crate::io::read_lines;

use itertools::Itertools;

crate::test::test_part!(test1, part1, 435);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> u32 {
	let maze = read_maze();
	let n = maze.len();
	let mut distances = vec![vec![u32::MAX; n]; n];
	distances[0][0] = 0;
	for (i, row) in maze.iter().enumerate() {
		for (j, _) in row.iter().enumerate() {
			if i < n - 1 {
				distances[i + 1][j] = distances[i + 1][j].min(distances[i][j] + maze[i + 1][j]);
			}
			if j < n - 1 {
				distances[i][j + 1] = distances[i][j + 1].min(distances[i][j] + maze[i][j + 1]);
			}
		}
	}
	distances
		.into_iter()
		.last()
		.unwrap()
		.into_iter()
		.last()
		.unwrap()
}

pub fn part2() -> u32 {
	let mut lines = read_lines("input/2021/15.txt");
	0
}

fn read_maze() -> Vec<Vec<u32>> {
	read_lines("input/2021/15.txt")
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
		.collect_vec()
}
