use crate::io::read_lines;

use std::collections::BinaryHeap;

crate::test::test_part!(test1, part1, 452);
crate::test::test_part!(test2, part2, 1263735);

pub fn part1() -> i64 {
	let field: Vec<Vec<i64>> = read_lines("input/2021/9.txt")
		.map(|line| {
			line.chars()
				.map(|c| c.to_digit(10).unwrap() as i64)
				.collect()
		})
		.collect();
	let mut sum = 0;
	for i in 0..field.len() {
		for j in 0..field[i].len() {
			let height = field[i][j];
			if (i == 0 || field[i - 1][j] > height)
				&& (i == field.len() - 1 || field[i + 1][j] > height)
				&& (j == 0 || field[i][j - 1] > height)
				&& (j == field[i].len() - 1 || field[i][j + 1] > height)
			{
				sum += height + 1
			}
		}
	}
	sum
}

pub fn part2() -> i64 {
	let field: Vec<Vec<i64>> = read_lines("input/2021/9.txt")
		.map(|line| {
			line.chars()
				.map(|c| c.to_digit(10).unwrap() as i64)
				.collect()
		})
		.collect();

	let mut visited: Vec<Vec<bool>> = field
		.iter()
		.map(|row| row.iter().map(|&space| space == 9).collect())
		.collect();

	let mut basin_sizes: BinaryHeap<i64> = BinaryHeap::new();
	for i in 0..field.len() {
		for j in 0..field[i].len() {
			if !visited[i][j] {
				basin_sizes.push(dfs(&field, &mut visited, i, j));
			}
		}
	}

	basin_sizes.iter().take(3).product()
}

fn dfs(field: &[Vec<i64>], visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> i64 {
	if visited[i][j] {
		return 0;
	}
	visited[i][j] = true;

	let mut sum = 1;
	if i > 0 {
		sum += dfs(field, visited, i - 1, j)
	};
	if i < field.len() - 1 {
		sum += dfs(field, visited, i + 1, j)
	};
	if j > 0 {
		sum += dfs(field, visited, i, j - 1)
	};
	if j < field[i].len() - 1 {
		sum += dfs(field, visited, i, j + 1)
	};
	sum
}
