use aoc::input;

aoc::test::test_part!(test1, part1, 252);
aoc::test::test_part!(test2, part2, 2608962048);

pub fn part1() -> usize {
	input!()
		.lines()
		.enumerate()
		.filter(|(i, line)| {
			let line = line.as_bytes();
			line[(3 * i) % line.len()] == b'#'
		})
		.count()
}

pub fn part2() -> usize {
	let lines: Vec<_> = input!().lines().collect();
	[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
		.into_iter()
		.map(|(right, down)| {
			lines
				.iter()
				.step_by(down)
				.enumerate()
				.filter(|(i, line)| {
					let line = line.as_bytes();
					line[(right * i) % line.len()] == b'#'
				})
				.count()
		})
		.product()
}
