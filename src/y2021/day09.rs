use crate::{io::read_lines, solution::Solution};

use std::collections::BinaryHeap;

pub struct Day09;

impl Solution for Day09 {
	fn year(&self) -> u32 {
		2021
	}

	fn day(&self) -> u32 {
		9
	}

	fn part1(&self) -> i64 {
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

	fn part2(&self) -> i64 {
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() {
		assert_eq!(452, Day09.part1());
	}

	#[test]
	fn part2() {
		assert_eq!(1263735, Day09.part2());
	}
}
