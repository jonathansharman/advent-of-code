use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let grid: Vec<Vec<i32>> = read_lines("input/2022/08.txt")
		.map(|line| line.as_bytes().iter().map(|c| (c - b'0') as i32).collect())
		.collect();
	let (nrows, ncols) = (grid.len(), grid[0].len());
	let mut visible: Vec<Vec<bool>> = vec![vec![false; ncols]; nrows];
	for row in 0..nrows {
		let mut max = -1;
		for col in 0..ncols {
			if grid[row][col] > max {
				max = grid[row][col];
				visible[row][col] = true;
			}
		}
		let mut max = -1;
		for col in (0..ncols).rev() {
			if grid[row][col] > max {
				max = grid[row][col];
				visible[row][col] = true;
			}
		}
	}
	for col in 0..ncols {
		let mut max = -1;
		for row in 0..nrows {
			if grid[row][col] > max {
				max = grid[row][col];
				visible[row][col] = true;
			}
		}
		let mut max = -1;
		for row in (0..nrows).rev() {
			if grid[row][col] > max {
				max = grid[row][col];
				visible[row][col] = true;
			}
		}
	}
	visible
		.into_iter()
		.map(|row| row.into_iter().map_into::<usize>().sum::<usize>())
		.sum()
}

pub fn part2() -> usize {
	0
}
