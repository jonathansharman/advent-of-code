use crate::io::read_lines;

crate::test::test_part!(test1, part1, 1757);
crate::test::test_part!(test2, part2, 422);

pub fn part1() -> i64 {
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

pub fn part2() -> i64 {
	let mut grid: Vec<Vec<i64>> = read_lines("input/2021/11.txt")
		.map(|line| {
			line.chars()
				.map(|c| c.to_digit(10).unwrap() as i64)
				.collect()
		})
		.collect();
	let mut queue = Vec::new();
	let mut time = 0;
	loop {
		time += 1;
		let mut flashes = 0;
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
		if flashes == (grid.len() * grid[0].len()) as i64 {
			return time;
		}
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
