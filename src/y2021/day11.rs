use itertools::Itertools;

use crate::{io::read_lines, solution::Solution};

pub struct Day11;

impl Solution for Day11 {
	fn year(&self) -> u32 {
		2021
	}

	fn day(&self) -> u32 {
		11
	}

	fn part1(&self) -> i64 {
		let mut grid: Vec<Vec<i64>> = read_lines("input/2021/11.txt")
			.map(|line| {
				line.chars()
					.map(|c| c.to_digit(10).unwrap() as i64)
					.collect()
			})
			.collect();
		let mut flashes = 0;
		let mut queue = Vec::new();
		for _ in 0..100 {
			for i in 0..grid.len() {
				for j in 0..grid[i].len() {
					process(&mut grid, &mut queue, &mut flashes, i, j);
				}
			}
			while !queue.is_empty() {
				let (i, j) = queue.pop().unwrap();
				if i > 0 {
					process(&mut grid, &mut queue, &mut flashes, i - 1, j);
					if j > 0 {
						process(&mut grid, &mut queue, &mut flashes, i - 1, j - 1);
					}
					if j < grid.len() - 1 {
						process(&mut grid, &mut queue, &mut flashes, i - 1, j + 1);
					}
				}
				if i < grid.len() - 1 {
					process(&mut grid, &mut queue, &mut flashes, i + 1, j);
					if j > 0 {
						process(&mut grid, &mut queue, &mut flashes, i + 1, j - 1);
					}
					if j < grid.len() - 1 {
						process(&mut grid, &mut queue, &mut flashes, i + 1, j + 1);
					}
				}
				if j > 0 {
					process(&mut grid, &mut queue, &mut flashes, i, j - 1);
				}
				if j < grid.len() - 1 {
					process(&mut grid, &mut queue, &mut flashes, i, j + 1);
				}
			}
			for row in grid.iter_mut() {
				for octopus in row {
					if octopus > &mut 9 {
						*octopus = 0;
					}
				}
			}
		}
		flashes
	}

	fn part2(&self) -> i64 {
		0
	}
}

fn process(
	grid: &mut Vec<Vec<i64>>,
	queue: &mut Vec<(usize, usize)>,
	flashes: &mut i64,
	i: usize,
	j: usize,
) {
	grid[i][j] += 1;
	if grid[i][j] == 10 {
		*flashes += 1;
		queue.push((i, j));
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() {
		assert_eq!(1757, Day11.part1());
	}

	#[test]
	fn part2() {
		assert_eq!(0, Day11.part2());
	}
}
