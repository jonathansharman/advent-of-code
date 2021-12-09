use crate::{io::read_lines, solution::Solution};

use std::collections::BinaryHeap;

pub struct Day9;

impl Solution for Day9 {
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
				let x = field[i][j];
				if (i == 0 || field[i - 1][j] > x)
					&& (i == field.len() - 1 || field[i + 1][j] > x)
					&& (j == 0 || field[i][j - 1] > x)
					&& (j == field[i].len() - 1 || field[i][j + 1] > x)
				{
					sum += x + 1
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

		basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap()
	}
}

fn dfs(field: &[Vec<i64>], visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> i64 {
	if visited[i][j] {
		return 0;
	}
	visited[i][j] = true;
	let up = if i == 0 {
		0
	} else {
		dfs(field, visited, i - 1, j)
	};
	let down = if i == field.len() - 1 {
		0
	} else {
		dfs(field, visited, i + 1, j)
	};
	let left = if j == 0 {
		0
	} else {
		dfs(field, visited, i, j - 1)
	};
	let right = if j == field[i].len() - 1 {
		0
	} else {
		dfs(field, visited, i, j + 1)
	};
	1 + up + down + left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() {
		assert_eq!(452, Day9.part1());
	}

	#[test]
	fn part2() {
		assert_eq!(1263735, Day9.part2());
	}
}
