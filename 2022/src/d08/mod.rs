use itertools::Itertools;

aoc::test::test_part!(test1, part1, 1798);
aoc::test::test_part!(test2, part2, 259308);

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> usize {
	let grid: Vec<Vec<i32>> = INPUT
		.lines()
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
	let grid: Vec<Vec<i32>> = INPUT
		.lines()
		.map(|line| line.as_bytes().iter().map(|c| (c - b'0') as i32).collect())
		.collect();
	let (nrows, ncols) = (grid.len(), grid[0].len());
	let mut max = 0;
	for (r, row) in grid.iter().enumerate().skip(1).take(nrows - 2) {
		for (c, &h) in row.iter().enumerate().skip(1).take(ncols - 2) {
			let mut c2 = c - 1;
			let left = loop {
				if c2 == 0 || row[c2] >= h {
					break c - c2;
				}
				c2 -= 1;
			};
			let mut c2 = c + 1;
			let right = loop {
				if c2 == ncols - 1 || row[c2] >= h {
					break c2 - c;
				}
				c2 += 1;
			};
			let mut r2 = r - 1;
			let up = loop {
				if r2 == 0 || grid[r2][c] >= h {
					break r - r2;
				}
				r2 -= 1;
			};
			let mut r2 = r + 1;
			let down = loop {
				if r2 == nrows - 1 || grid[r2][c] >= h {
					break r2 - r;
				}
				r2 += 1;
			};
			max = max.max(left * right * up * down);
		}
	}
	max
}
