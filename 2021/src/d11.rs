use aoc::grid::{Grid, Point};

aoc::test::test_part!(test1, part1, 1757);
aoc::test::test_part!(test2, part2, 422);

const INPUT: &str = include_str!("input/11.txt");
const WIDTH: usize = 10;

pub fn part1() -> usize {
	let mut grid: Grid<u32> = INPUT
		.lines()
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
		.collect();
	let mut flashes = 0;
	let mut queue = Vec::new();
	for _ in 0..100 {
		for i in 0..WIDTH {
			for j in 0..WIDTH {
				process(&mut grid, &mut queue, &mut flashes, (i, j).into());
			}
		}
		while let Some(coords) = queue.pop() {
			for coords in grid
				.eight_neighbors(coords)
				.map(|(coords, _)| coords)
				.collect::<Vec<_>>()
			{
				process(&mut grid, &mut queue, &mut flashes, coords);
			}
		}
		for row in grid.rows_mut() {
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
	let mut grid: Grid<u32> = INPUT
		.lines()
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
		.collect();
	let mut queue = Vec::new();
	let mut time = 0;
	loop {
		time += 1;
		let mut flashes = 0;
		for i in 0..WIDTH {
			for j in 0..WIDTH {
				process(&mut grid, &mut queue, &mut flashes, (i, j).into());
			}
		}
		while let Some(coords) = queue.pop() {
			for coords in grid
				.eight_neighbors(coords)
				.map(|(coords, _)| coords)
				.collect::<Vec<_>>()
			{
				process(&mut grid, &mut queue, &mut flashes, coords);
			}
		}
		for row in grid.rows_mut() {
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
	grid: &mut Grid<u32>,
	queue: &mut Vec<Point>,
	flashes: &mut usize,
	point: Point,
) {
	grid[point] += 1;
	if grid[point] == 10 {
		*flashes += 1;
		queue.push(point);
	}
}
