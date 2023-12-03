use crate::{io::read_lines, neighbors};

crate::test::test_part!(test1, part1, 1757);
crate::test::test_part!(test2, part2, 422);

const WIDTH: usize = 10;

pub fn part1() -> usize {
	let mut grid: Vec<Vec<u32>> = read_lines("input/2021/11.txt")
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
		.collect();
	let mut flashes = 0;
	let mut queue = Vec::new();
	for _ in 0..100 {
		for i in 0..WIDTH {
			for j in 0..WIDTH {
				process(&mut grid, &mut queue, &mut flashes, i, j);
			}
		}
		while let Some((i, j)) = queue.pop() {
			for (i, j) in neighbors::eight(WIDTH, WIDTH, i, j) {
				process(&mut grid, &mut queue, &mut flashes, i, j);
			}
		}
		for row in grid.iter_mut() {
			for octopus in row {
				if *octopus > 9 {
					*octopus = 0;
				}
			}
		}
	}
	flashes
}

pub fn part2() -> i64 {
	let mut grid: Vec<Vec<u32>> = read_lines("input/2021/11.txt")
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
		.collect();
	let mut queue = Vec::new();
	let mut time = 0;
	loop {
		time += 1;
		let mut flashes = 0;
		for i in 0..WIDTH {
			for j in 0..WIDTH {
				process(&mut grid, &mut queue, &mut flashes, i, j);
			}
		}
		while let Some((i, j)) = queue.pop() {
			for (i, j) in neighbors::eight(WIDTH, WIDTH, i, j) {
				process(&mut grid, &mut queue, &mut flashes, i, j);
			}
		}
		for row in grid.iter_mut() {
			for octopus in row {
				if *octopus > 9 {
					*octopus = 0;
				}
			}
		}
		if flashes == (WIDTH * WIDTH) {
			return time;
		}
	}
}

fn process(
	grid: &mut [Vec<u32>],
	queue: &mut Vec<(usize, usize)>,
	flashes: &mut usize,
	i: usize,
	j: usize,
) {
	grid[i][j] += 1;
	if grid[i][j] == 10 {
		*flashes += 1;
		queue.push((i, j));
	}
}
