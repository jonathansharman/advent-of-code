use std::collections::BinaryHeap;

use aoc::input;

aoc::test::test_part!(test1, part1, 452);
aoc::test::test_part!(test2, part2, 1263735);

pub fn part1() -> u32 {
	let field: Vec<Vec<u32>> = input!()
		.lines()
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
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

pub fn part2() -> u32 {
	let mut field: Vec<Vec<u32>> = input!()
		.lines()
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
		.collect();

	let mut basin_sizes: BinaryHeap<u32> = BinaryHeap::new();
	for i in 0..field.len() {
		for j in 0..field[i].len() {
			if field[i][j] != 9 {
				basin_sizes.push(dfs(&mut field, i, j));
			}
		}
	}

	basin_sizes.iter().take(3).product()
}

fn dfs(field: &mut [Vec<u32>], i: usize, j: usize) -> u32 {
	if field[i][j] == 9 {
		return 0;
	}
	field[i][j] = 9;

	let mut sum = 1;
	if i > 0 {
		sum += dfs(field, i - 1, j)
	};
	if i < field.len() - 1 {
		sum += dfs(field, i + 1, j)
	};
	if j > 0 {
		sum += dfs(field, i, j - 1)
	};
	if j < field[i].len() - 1 {
		sum += dfs(field, i, j + 1)
	};
	sum
}
